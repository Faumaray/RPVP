use mpi::topology::SystemCommunicator;
use mpi::traits::{Communicator, CommunicatorCollectives, Destination, Root, Source};

use crate::lab_third::Matrix;

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
    fn get_distribution(&self, rows: usize, columns: usize) -> (Vec<i32>, Vec<i32>) {
        use mpi::Count;
        println!(
            "{}/{}={}",
            rows,
            self.size as usize,
            rows % self.size as usize
        );
        let counts: Vec<Count> = {
            let mut tmp = Vec::new();
            if rows % self.size as usize == 0 {
                let count = (rows as i32 / self.size) * columns as i32;
                println!("count = {}", count);
                for _ in 0..self.size {
                    tmp.push(count);
                }
            } else {
                let count = ((rows as i32 - (rows % self.size as usize) as i32) / self.size)
                    * columns as i32;
                let remain = (rows as i32 % self.size) * columns as i32;
                println!("count = {}", count);
                println!("remain = {}", remain);
                for _ in 0..self.size - 1 {
                    tmp.push(count);
                }
                tmp.push(count + remain);
            }
            tmp.shrink_to_fit();
            tmp
        };
        let displs: Vec<i32> = counts
            .iter()
            .scan(0, |acc, &x| {
                let tmp = *acc;
                *acc += x;
                Some(tmp)
            })
            .collect();
        println!("{}", displs.last().unwrap());
        (displs, counts)
    }

    fn sgemv(&self, generate: bool, rows: usize, columns: usize) -> Vec<T> {
        use mpi::datatype::Partition;
        use mpi::Count;
        let mut result: Vec<T> = vec![T::default(); columns];
        let mut local_vector: Vec<T> = vec![T::default(); columns];
        let local_matrix_flatten = if self.rank == 0 {
            let (mut matrix, vector) = Executor::generate_test_data(rows, columns, generate);
            local_vector = vector;
            let (displs, counts) = self.get_distribution(rows, columns);
            // let partition = Partition::new(&matrix, counts, displs);
            for rank in 1..self.size {
                self.communicator.process_at_rank(rank).send(
                    &matrix
                        .drain(0..counts[rank as usize] as usize)
                        .collect::<Vec<T>>(),
                );
                matrix.shrink_to_fit();
                println!("0 send to {}", rank);
            }
            matrix
        } else {
            self.communicator.process_at_rank(0).receive_vec().0
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
