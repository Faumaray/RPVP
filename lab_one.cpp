#include "lab_one.hpp"
void alltoall(size_t size) {
    // Get the number of processes
    int world_size = MPI::COMM_WORLD.Get_size();
    // double starttime, endtime;
    // Get the rank of the process
    int world_rank = MPI::COMM_WORLD.Get_rank();
    char *rbuf = (char *)calloc(size * world_size - 1, sizeof(char));
    double starttime, endtime;

    MPI::Status *status = new MPI::Status[world_size];
    /* Request objects for non-blocking send and receive */
    MPI::Request *send_req = new MPI::Request[world_size];
    MPI::Request *recv_req = new MPI::Request[world_size];
    char *sbuf = (char *)malloc(size);
    for (size_t i = 0; i < size; i++) {
        sbuf[i] = (char)rand() % 256;
    }

    starttime = MPI::Wtime();
    for (int i = 0; i < world_size; i++) {
        send_req[i] = MPI::COMM_WORLD.Isend(sbuf, size, MPI::CHAR, i, 0);
    }
    MPI::Request::Waitall(world_size, send_req, status);
    for (int i = 0; i < world_size; i++) {

        recv_req[i] =
            MPI::COMM_WORLD.Isend(rbuf + (size * i), size, MPI::CHAR, i, 0);
    }
    MPI::Request::Waitall(world_size, recv_req, status);

    endtime = MPI::Wtime();
    printf("Time %f of process %d\n", endtime - starttime, world_rank);
    free(sbuf);
    free(rbuf);
}

void gather(size_t size) {
    int world_size = MPI::COMM_WORLD.Get_size();
    // double starttime, endtime;
    // Get the rank of the process
    int world_rank = MPI::COMM_WORLD.Get_rank();
    char *rbuf = (char *)calloc(size * world_size - 1, sizeof(char));
    double starttime, endtime;
    starttime = MPI::Wtime();
    if (world_rank != 0) {
        char *sbuf = (char *)malloc(size);
        for (size_t i = 0; i < size; i++) {
            sbuf[i] = (char)rand() % 256;
        }
        MPI::COMM_WORLD.Send(sbuf, size, MPI::CHAR, 0, 0);
        free(sbuf);
    } else {
        for (int i = 1; i < world_size; i++) {
            if (i != world_rank) {
                MPI::COMM_WORLD.Recv(rbuf + (size * i), size, MPI::CHAR, i, 0);

                printf(
                    "Process %d received in element %lu data %hhd from %d \n",
                    world_rank, size * i, rbuf[size * i], i);

                endtime = MPI::Wtime();
                printf("Time %f of process %d\n", endtime - starttime,
                       world_rank);
            }
        }
        free(rbuf);
    }
}

void broadcast(size_t size) {
    // Get the number of processes
    int world_size = MPI::COMM_WORLD.Get_size();
    // double starttime, endtime;
    // Get the rank of the process
    int world_rank = MPI::COMM_WORLD.Get_rank();
    char *rbuf = (char *)calloc(size, sizeof(char));

    double starttime, endtime;
    starttime = MPI::Wtime();
    if (world_rank == 0) {
        char *sbuf = (char *)malloc(size);
        for (size_t i = 0; i < size; i++) {
            sbuf[i] = (char)rand() % 256;
        }
        for (int i = 0; i < world_size; i++) {
            if (i != world_rank) {
                MPI::COMM_WORLD.Send(sbuf, size, MPI::CHAR, i, 0);
            }
        }
        free(sbuf);
    } else {
        MPI::COMM_WORLD.Recv(rbuf, size, MPI::CHAR, 0, 0);
        printf("Process %d received data(first element) %hhd from %d \n",
               world_rank, rbuf[0], 0);
        free(rbuf);
        endtime = MPI::Wtime();
        printf("Time %f of process %d\n", endtime - starttime, world_rank);
    }
}

void ring(size_t size) {
    // Get the number of processes
    int world_size = MPI::COMM_WORLD.Get_size();
    // double starttime, endtime;
    // Get the rank of the process
    int world_rank = MPI::COMM_WORLD.Get_rank();
    double starttime, endtime;
    char *token = (char *)calloc(size, sizeof(char));
    if (world_rank != 0) {
        MPI::COMM_WORLD.Recv(token, size, MPI::CHAR, world_rank - 1, 0);
        printf("Process %d received token size %lu from process %d\n",
               world_rank, sizeof(token), world_rank - 1);
        printf("----------------------------------\n");
    } else {
        for (size_t i = 0; i < size; i++) {
            token[i] = (char)rand() % 256;
        }
        starttime = MPI::Wtime();
    }
    printf("Process %d send token size %lu \n", world_rank, sizeof(token));
    MPI::COMM_WORLD.Send(token, size, MPI::CHAR, (world_rank + 1) % world_size,
                         0);
    // Now process 0 can receive from the last process.
    if (world_rank == 0) {
        MPI::COMM_WORLD.Recv(token, size, MPI::CHAR, world_size - 1, 0);
        printf("Process %d received token size %lu from process %d\n",
               world_rank, sizeof(token), world_size - 1);
        printf("----------------------------------\n");
        endtime = MPI::Wtime();
        printf("That took %f seconds\n", endtime - starttime);
        free(token);
    }
}
