use log::{debug, info, trace};
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
    pub fn topological_ring(&self)
    {
        let mut send = -1;
        let cart = self.communicator().create_cartesian_communicator(&[0], &[true], true).unwrap();

        let (source, dest) = cart.shift(0, 1);
        if self.rank() == 0 {
            cart.process_at_rank(dest.unwrap()).send(&self.rank());
            send = cart.process_at_rank(source.unwrap()).receive().0
        } else {
            send = cart.process_at_rank(source.unwrap()).receive().0;
            cart.process_at_rank(dest.unwrap()).send(&self.rank());
        }
        
        info!("rank={} B={}", self.rank(), send);
        
        
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
        debug!(
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
        trace!("[{}] Entered in SGEMV", self.rank);
        let mut result: Vec<T> = vec![T::default(); columns];
        let (mut matrix, mut vector) =
            Executor::generate_test_data(rows, columns, generate, self.rank);
        trace!("[{}] Generated Matrix and Vector", self.rank);
        let mut t_start = 0.0;
        if self.rank == 0 {
            let counts = self.get_distribution(rows, columns);
            mpi::request::multiple_scope(self.size as usize, |scope, col| {
                trace!("[{}] Entered first scope", self.rank);
                trace!("[{}] Calculated distribution", self.rank);
                debug!("Vector: {}", vector[0]);
                t_start = mpi::time();
                for rank in 1..self.size {
                    let prev = counts[rank as usize - 1] as usize;
                    col.add(self.communicator.process_at_rank(rank).immediate_send(
                        scope,
                        &matrix[prev..counts[rank as usize - 1] as usize + prev],
                    ));
                    info!("[{}] Send slice of matrix to {}", self.rank, rank);
                }
                col.wait_all(&mut Vec::new());
            });
            trace!("[{}] Exited first scope", self.rank);
            matrix = matrix[0..counts[0] as usize].to_vec();
        } else {
            matrix = self.communicator.process_at_rank(0).receive_vec().0;
        }
        trace!("[{}] Get local matrix slice", self.rank);

        debug!(
            "[{}]Vector length before broadcast: {}",
            self.rank,
            vector.len()
        );
        self.communicator
            .process_at_rank(0)
            .broadcast_into(&mut vector);
            debug!(
            "[{}]Vector length after broadcast: {}",
            self.rank,
            vector.len()
        );

        let mut column = 0;
        let mut row: usize = 0;

        let mut local_value: Vec<T> = vec![T::default(); columns];
        for value in matrix {
            local_value[column] += vector[column] * value;
            column += 1;
            if column == columns {
                column = 0;
                row += 1;
                trace!("[{}] Computed {} row: {}", self.rank, row, local_value[0])
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
