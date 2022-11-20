use std::ops::Range;

use log::info;
use mpi::traits::{Communicator, CommunicatorCollectives, Root};
use num::Signed;
pub use num_traits::Num;
use rand::distributions::uniform::SampleUniform;

use crate::executor::Executor;

pub trait LabTwo<T>
where
    T: Num
        + std::fmt::Display
        + Signed
        + Copy
        + Default
        + SampleUniform
        + std::clone::Clone
        + std::ops::AddAssign
        + num::FromPrimitive
        + mpi::traits::Equivalence
        + std::cmp::PartialOrd
        + std::ops::MulAssign,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    f32: From<T>,
{
    fn monte_carlo(
        executor: Executor,
        f: fn(T, T) -> Option<T>,
        x_range: Range<T>,
        y_range: Range<T>,
        n: i32,
    ) -> T {
        use rand::prelude::*;
        let mut rand = rand::thread_rng();
        let mut result = T::from_i32(0).unwrap();
        let t_start = mpi::time();
        let mut i_n = 0;
        let mut s = T::from_i32(0).unwrap();
        for _ in (executor.rank()..n).step_by(executor.size() as usize) {
            i_n += 1;
            if let Some(value) = f(
                rand.gen_range(x_range.clone()),
                rand.gen_range(y_range.clone()),
            ) {
                s += value;
            }
        }
        if executor.rank() != 0 {
            executor
                .communicator()
                .process_at_rank(0)
                .reduce_into(&s, mpi::collective::SystemOperation::sum());
            executor
                .communicator()
                .process_at_rank(0)
                .reduce_into(&i_n, mpi::collective::SystemOperation::sum());
        } else {
            let mut gin = T::from_i32(0).unwrap();
            let mut gsum = T::from_i32(0).unwrap();
            executor.communicator().process_at_rank(0).reduce_into_root(
                &s,
                &mut gsum,
                mpi::collective::SystemOperation::sum(),
            );
            executor.communicator().process_at_rank(0).reduce_into_root(
                &i_n,
                &mut gin,
                mpi::collective::SystemOperation::sum(),
            );
            let v = x_range.end * gin / T::from_i32(n).unwrap();
            result = v * gsum / gin;
            info!("Result: {}, n {}", result, n);
            info!("Time estimated: {}", mpi::time() - t_start);
        }

        result
    }

    fn midpoint_rule(executor: Executor, e: f32, f: fn(T) -> T, bound: (T, T)) -> T {
        let mut result = T::from_i32(0).unwrap();
        let t_start = mpi::time();
        let mut n = 1;
        let mut sq: [T; 2] = [T::from_i32(0).unwrap(), T::from_i32(0).unwrap()];
        let mut delta: f32 = 1.0;
        let mut k = 0;
        while delta > e {
            let points_per_proc = n / executor.size();
            let lb = executor.rank() * points_per_proc;
            let ub = if executor.rank() == executor.size() - 1 {
                n - 1
            } else {
                lb + points_per_proc - 1
            };
            let h = (bound.1 - bound.0) / T::from_i32(n).unwrap();
            let mut s = T::from_i32(0).unwrap();
            for i in lb..=ub {
                s += f(bound.0 + h * T::from_f32(i as f32 + 0.5).unwrap());
            }
            executor.communicator().all_reduce_into(
                &s,
                &mut sq[k as usize],
                mpi::collective::SystemOperation::sum(),
            );
            sq[k as usize] *= h;
            if n > 1 {
                delta = <T as Into<f32>>::into((sq[k as usize] - sq[k as usize ^ 1]).abs()) / 3.0;
            }
            n *= 2;
            k ^= 1;
        }

        if executor.rank() == 0 {
            info!(
                "Result Pi: {}; Runge rule: EPS {}, n {}",
                sq[k] * sq[k],
                e,
                n / 2
            );
            info!("Time estimated: {}\n", mpi::time() - t_start);
            result = sq[k] * sq[k];
        }
        result
    }
}
