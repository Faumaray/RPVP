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
    pub fn topological_ring(&self, dims: usize)
    {
        use rand::prelude::*;
        let mut send: u32 = rand::thread_rng().gen_range(0..300);
        let mut distribution: Vec<i32> = vec![0; dims];
        let t_start: f64 = mpi::time();
        unsafe{
            mpi::ffi::MPI_Dims_create(self.size(), dims as i32, distribution.as_mut_ptr());
        }
        let cart = self.communicator().create_cartesian_communicator(&distribution, &vec![true; distribution.len()], true).unwrap();
        let (mut source, mut dest) = cart.shift(0, 1);
        if self.rank() == 0 {    
            cart.process_at_rank(dest.unwrap()).send(&send);
            info!("[{}] Sending {} to {}", self.rank(), send, dest.unwrap());
           /* if self.rank() != 0 {
                info!("[{}] Sending {} to {}", self.rank(), send, 0);
                cart.process_at_rank(0).send(&send);
                send = cart.process_at_rank(source.unwrap()).receive().0;
                info!("[{}] Got {} from {}", self.rank(), send, source.unwrap());
            }*/
        } else {
            let cords = cart.rank_to_coordinates(self.rank());
            if cords[0] == dims as i32 - 1 && cords[1] == dims as i32 - 1 {//[last, last]
                dest = Some(0);
                send = cart.process_at_rank(source.unwrap()).receive().0;
                info!("[{}] Got {} from {}", self.rank(), send, source.unwrap());
                let add: u32 = rand::thread_rng().gen_range(0..100);
                info!("[{}] Adding own data {}+{}={}",self.rank(),send, add,send + add);
                info!("[{}] Sending {} to {}", self.rank(), send + add, dest.unwrap());
                cart.process_at_rank(dest.unwrap()).send(&(send + add));
            } else if cords[0] == dims as i32 - 1 && cords[1] != dims as i32 - 1{ //[last, any] WORK
                dest = Some(cart.coordinates_to_rank(&[0,cords[1] + 1]));
                send = cart.process_at_rank(source.unwrap()).receive().0;
                info!("[{}] Got {} from {}", self.rank(), send, source.unwrap());
                let add: u32 = rand::thread_rng().gen_range(0..100);
                info!("[{}] Adding own data {}+{}={}",self.rank(),send, add,send + add);
                info!("[{}] Sending {} to {}", self.rank(), send + add, dest.unwrap());
                cart.process_at_rank(dest.unwrap()).send(&(send + add));
                info!("[{}] Sending {} to {}", self.rank(), send + add, 0);
                cart.process_at_rank(0).send(&(send + add));
            }else if cords[0] == 0 { //[first,any]
                source = Some(cart.coordinates_to_rank(&[dims as i32-1, cords[1]-1]));
                send = cart.process_at_rank(source.unwrap()).receive().0;
                info!("[{}] Got {} from {}", self.rank(), send, source.unwrap());
                let add: u32 = rand::thread_rng().gen_range(0..100);
                info!("[{}] Adding own data {}+{}={}",self.rank(),send, add,send + add);
                info!("[{}] Sending {} to {}", self.rank(), send + add, dest.unwrap());
                cart.process_at_rank(dest.unwrap()).send(&(send + add));
                info!("[{}] Sending {} to {}", self.rank(), send + add, 0);
                cart.process_at_rank(0).send(&(send + add));
            }   else { //[any,any]
                send = cart.process_at_rank(source.unwrap()).receive().0;
                info!("[{}] Got {} from {}", self.rank(), send, source.unwrap());
                let add: u32 = rand::thread_rng().gen_range(0..100);
                info!("[{}] Adding own data {}+{}={}",self.rank(),send, add,send + add);
                info!("[{}] Sending {} to {}", self.rank(), send + add, dest.unwrap());
                cart.process_at_rank(dest.unwrap()).send(&(send + add));
                info!("[{}] Sending {} to {}", self.rank(), send + add, 0);
                cart.process_at_rank(0).send(&(send + add));
            }

            
           
           /*  cart.process_at_rank(dest.unwrap()).send(&(send+add));
            if dest.unwrap() != 0 {
                info!("[{}] Sending {} to {}", self.rank(), send + add, 0);
                cart.process_at_rank(0).send(&(send+add));
            }*/
        }

        if self.rank() == 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let mut result: Vec<(i32, u32)> = vec![(0,0); self.size() as usize - 1];   
            for rank in 1..self.size() {
                result[rank as usize - 1] = (rank, cart.process_at_rank(rank).receive().0);
                info!("[{}] Got {} from {}", self.rank(), result[rank as usize - 1].1, rank);
            }
            info!("[{}] All results = {:?}", self.rank(), result);
            info!("Time estimated = {}", mpi::time()-t_start - 1.0);
        } else {
            info!("Time estimated = {}", mpi::time()-t_start);
        }
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
