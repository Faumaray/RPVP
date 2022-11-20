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

    fn sgemv(&self, generate: bool, rows: usize, columns: usize) -> Vec<T> {
        let mut result: Vec<T> = vec![T::default(); columns];

        let (local_matrix_flatten, mut local_vector) = if self.rank == 0 {
            let (mut matrix, vector) = Executor::generate_test_data(rows, columns, generate);
            let counts = self.get_distribution(rows, columns);
            // TODO!: Find way to use ISEND
            for rank in 1..self.size {
                self.communicator
                    .process_at_rank(rank)
                    .send(matrix.drain(0..counts[rank as usize] as usize).as_slice());
                info!("0 send to {}", rank);
            }
            (matrix, vector)
        } else {
            (
                self.communicator.process_at_rank(0).receive_vec().0,
                vec![T::default(); columns],
            )
        };
        self.communicator
            .process_at_rank(0)
            .broadcast_into(&mut local_vector);
        let mut column = 0;

        let mut local_value: Vec<T> = vec![T::default(); columns];
        for value in local_matrix_flatten {
            local_value[column] += local_vector[column] * value;
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

        result
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
