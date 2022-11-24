use log::{info, trace};
use mpi::topology::SystemCommunicator;
use mpi::traits::{Communicator, CommunicatorCollectives, Destination, Root, Source};

use crate::lab_two::LabTwo;

pub struct Executor {
    communicator: SystemCommunicator,
    size: i32,
    rank: i32,
}

impl Executor {
    pub fn new(communicator: SystemCommunicator) -> Self {
        Self {
            communicator,
            size: communicator.size(),
            rank: communicator.rank(),
        }
    }
    pub fn size(&self) -> i32 {
        self.size
    }
    pub fn rank(&self) -> i32 {
        self.rank
    }
    pub fn communicator(&self) -> SystemCommunicator {
        self.communicator
    }
}

impl<T> crate::lab_third::LabThree<T> for Executor
where
    T: num_traits::Num
        + std::fmt::Display
        + num::FromPrimitive
        + Default
        + Copy
        + std::clone::Clone
        + std::ops::AddAssign
        + mpi::traits::Equivalence,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    [T]: mpi::traits::Buffer
        + mpi::datatype::Pointer
        + mpi::traits::AsDatatype
        + mpi::traits::BufferMut,
    Vec<T>: mpi::traits::Buffer
        + mpi::datatype::Pointer
        + mpi::traits::AsDatatype
        + mpi::traits::BufferMut,
{
    fn get_distribution(&self, rows: usize, columns: usize) -> Vec<i32> {
        trace!(
            "{}/{}={}",
            rows,
            self.size as usize,
            rows % self.size as usize
        );
        if rows % self.size as usize == 0 {
            vec![(rows as i32 / self.size) * columns as i32; self.size as usize]
        } else {
            let mut tmp = vec![
                ((rows as i32 - (rows % self.size as usize) as i32) / self.size)
                    * columns as i32;
                self.size as usize
            ];
            tmp[self.size as usize - 1] += (rows as i32 % self.size) * columns as i32;
            tmp
        }
    }

    fn sgemv(&self, generate: bool, rows: usize, columns: usize, mut result: Vec<T>) {
        let (matrix, mut vector) = Executor::generate_test_data(rows, columns, generate, self.rank);
        let mut t_start = 0.0;
        mpi::request::multiple_scope(self.size as usize, |scope, col| {
            if self.rank == 0 {
                let counts = self.get_distribution(rows, columns);
                trace!("Vector: {}", vector[0]);
                // TODO!: Find way to use ISEND
                t_start = mpi::time();
                for rank in 0..self.size {
                    col.add(
                        self.communicator
                            .process_at_rank(rank)
                            .immediate_send(scope, &matrix[0..counts[rank as usize] as usize]),
                    );
                    info!("0 send to {}", rank);
                }
                col.wait_all(&mut Vec::new());
            }
            let matrix = self.communicator.process_at_rank(0).receive_vec().0;
            trace!("[{}]Vector: {}", self.rank, vector[0]);
            self.communicator
                .process_at_rank(0)
                .broadcast_into(&mut vector);
            trace!("[{}]Vector: {}", self.rank, vector[0]);

            let mut column = 0;

            let mut local_value: Vec<T> = vec![T::default(); columns];
            for value in matrix {
                local_value[column] += vector[column] * value;
                column += 1;
                if column == columns {
                    column = 0;
                }
            }

            self.communicator.all_reduce_into(
                &local_value,
                &mut result,
                mpi::collective::SystemOperation::sum(),
            );
            if self.rank == 0 {
                info!("Time: {}", mpi::time() - t_start);
                info!("First value: {} ", result[0]);
            }
        });
    }
}

impl<T> LabTwo<T> for Executor
where
    T: num::Num
        + std::fmt::Display
        + num::Signed
        + Copy
        + Default
        + rand::distributions::uniform::SampleUniform
        + std::clone::Clone
        + std::ops::AddAssign
        + num::FromPrimitive
        + mpi::traits::Equivalence
        + std::cmp::PartialOrd
        + std::ops::MulAssign,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    f32: From<T>,
{
}
