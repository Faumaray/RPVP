pub use num_traits::Num;
use rand::distributions::Standard;

pub trait LabThree<T>
where
    T: Num + Copy + Default + std::clone::Clone + std::ops::AddAssign + num::FromPrimitive,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    Vec<T>: mpi::traits::Buffer
        + mpi::datatype::Pointer
        + mpi::traits::AsDatatype
        + mpi::traits::BufferMut,
{
    fn generate_test_data(
        rows: usize,
        columns: usize,
        randomize: bool,
        rank: i32,
    ) -> (Vec<T>, Vec<T>) {
        use rand::prelude::*;
        if rank == 0 {
            if randomize {
                (
                    rand::thread_rng()
                        .sample_iter(Standard)
                        .take(rows * columns)
                        .collect(),
                    rand::thread_rng()
                        .sample_iter(Standard)
                        .take(columns)
                        .collect(),
                )
            } else {
                (
                    vec![T::default() + T::from_i32(1).unwrap(); columns * rows],
                    vec![T::default() + T::from_i32(2).unwrap(); columns],
                )
            }
        } else {
            (Vec::new(), vec![T::default(); columns])
        }
    }
    fn get_distribution(&self, rows: usize, columns: usize) -> Vec<i32>;
    #[allow(patterns_in_fns_without_body)]
    fn sgemv(&self, generate: bool, rows: usize, columns: usize) -> Vec<T>;
}
