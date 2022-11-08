extern crate mpi;

use log::info;
use log::trace;
use mpi::traits::*;
pub fn ring(size: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let world_size = world.size();
    let world_rank = world.rank();
    let t_start = mpi::time();
    let msg: Vec<u8> = vec![0; size];
    if world_rank != 0 {
        let (rec_msg, _) = world.process_at_rank(world_rank - 1).receive_vec::<u8>();
        let t_end = mpi::time();
        info!(
            "Process {} received token size {} from process {} with time {}",
            world_rank,
            rec_msg.len(),
            world_rank - 1,
            t_end - t_start
        );
    } else {
        info!("Ring Method on {} processes with size {}", world_size, size);
    }
    world
        .process_at_rank((world_rank + 1) % world_size)
        .send::<Vec<u8>>(&msg);

    info!(
        "Process {} send token size {} to the {}",
        world_rank,
        msg.len(),
        (world_rank + 1) % world_size
    );

    if world_rank == 0 {
        let (rec_msg, _) = world.process_at_rank(world_size - 1).receive_vec::<u8>();
        let t_end = mpi::time();
        info!(
            "Process {} received token size {} from process {} with time {}",
            world_rank,
            rec_msg.len(),
            world_size - 1,
            t_end - t_start
        );
        info!(
            "-----------------------------------------------------------------------------------"
        );
    }
}

pub fn gather(size: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let world_size = world.size();
    let world_rank = world.rank();
    let t_start = mpi::time();
    let mut msg_recv: Vec<Vec<u8>> = vec![Vec::new(); world_size as usize - 1];
    if world_rank == 0 {
        info!(
            "Gather Method on {} processes with size {}",
            world_size, size
        );

        for rank in 1..world_size {
            if rank != world_rank {
                (msg_recv[rank as usize - 1], _) = world.process_at_rank(rank).receive_vec::<u8>();
                let t_end = mpi::time();
                info!(
                    "Process {} received token size of {} from {} with time {}",
                    world_rank,
                    msg_recv[rank as usize - 1].len(),
                    rank,
                    t_end - t_start
                );
            }
        }
        info!(
            "-----------------------------------------------------------------------------------"
        );
    } else {
        let msg_send: Vec<u8> = vec![1; size];
        world.process_at_rank(0).send(&msg_send);
        info!("Process {} send token size of {}", world_rank, size);
    }
}

pub fn broadcast(size: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let world_size = world.size();
    let world_rank = world.rank();
    let t_start = mpi::time();
    if world_rank == 0 {
        info!(
            "Broadcast method on {} processes with size {}",
            world_size, size
        );
        let msg_send: Vec<u8> = vec![0; size];
        for rank in 0..world_size {
            if rank != world_rank {
                world.process_at_rank(rank).send(&msg_send);
                info!(
                    "Process {} broadcasted msg to {} with size {}",
                    world_rank,
                    rank,
                    msg_send.len()
                );
            }
        }
    } else {
        let (msg_recv, _) = world.process_at_rank(0).receive_vec::<u8>();
        info!(
            "Process {} received token size of {} from {}",
            world_rank,
            msg_recv.len(),
            0
        );
        let t_end = mpi::time();
        info!("Time {} of process {}", t_end - t_start, world_rank);
    }
}

pub fn alltoall(size: usize) {
    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let world_size = world.size();
    let world_rank = world.rank();
    let t_start = mpi::time();
    let msg_send: Vec<u8> = vec![0; size];
    let mut msg_recv: Vec<Vec<u8>> = vec![vec![0; size]; world_size as usize];
    if world_rank == 0 {
        info!(
            "AllToAll method on {} processes with size {}",
            world_size, size
        );
    }
    mpi::request::multiple_scope(world_size as usize, |scope, coll| {
        for rank in 0..world_size {
            coll.add(world.process_at_rank(rank).immediate_send(scope, &msg_send));
            info!(
                "Process {} send token size of {}",
                world_rank,
                msg_send.len()
            );
        }
        coll.wait_all(&mut Vec::new());
        for (rank, recv) in msg_recv.iter_mut().enumerate() {
            coll.add(
                world
                    .process_at_rank(rank as i32)
                    .immediate_receive_into(scope, recv),
            );
        }
        coll.wait_all(&mut Vec::new());
    });
    let t_end = mpi::time();
    info!("Time {} of process {}", t_end - t_start, world_rank);
    if world_rank == 0 {
        info!(
            "-----------------------------------------------------------------------------------"
        );
    }
}
