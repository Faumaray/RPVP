#include "lab_one.hpp"
void alltoall(size_t size) {
  // Get the number of processes
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  // double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *rbuf = (char *)calloc(size * world_size - 1, sizeof(char));
  double starttime, endtime;

  MPI_Status  *status = new MPI_Status [world_size];
  /* Request objects for non-blocking send and receive */
  MPI_Request  *send_req = new MPI_Request [world_size];
  MPI_Request  *recv_req = new MPI_Request [world_size];
  char *sbuf = (char *)malloc(size);
  for (size_t i = 0; i < size; i++) {
    sbuf[i] = (char)rand() % 256;
  }

  starttime = MPI_Wtime();
  for (int i = 0; i < world_size; i++) {
    MPI_Isend(sbuf, size, MPI_CHAR, i, 0, MPI_COMM_WORLD, &send_req[i]);
    std::cout << "Process " << i << "send token size: " << size << std::endl;
  }
  MPI_Waitall(world_size, send_req, status);
  for (int i = 0; i < world_size; i++) {
    MPI_Irecv(rbuf + (size * i), size, MPI_CHAR, i, 0, MPI_COMM_WORLD,
              &recv_req[i]);
    std::cout << "Process " << i << "received token size: " << size
              << std::endl;
  }
  MPI_Waitall(world_size, recv_req, status);

  endtime = MPI_Wtime();
  std::cout << "Time " << endtime - starttime << " of process " << world_rank
            << std::endl;
  free(sbuf);
  free(rbuf);
}

void gather(size_t size) {
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  // double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *rbuf = (char *)calloc(size * world_size - 1, sizeof(char));
  double starttime, endtime;
  starttime = MPI_Wtime();
  if (world_rank != 0) {
    char *sbuf = (char *)malloc(size);
    for (size_t i = 0; i < size; i++) {
      sbuf[i] = (char)rand() % 256;
    }
    MPI_Send(sbuf, size, MPI_CHAR, 0, 0, MPI_COMM_WORLD);
    free(sbuf);
  } else {
    for (int i = 1; i < world_size; i++) {
      if (i != world_rank) {
        MPI_Recv(rbuf + (size * i), size, MPI_CHAR, i, 0, MPI_COMM_WORLD,
                 MPI_STATUS_IGNORE);
        printf(
            "Process %d received in element %lu size of data %ld from %d \n",
            world_rank, size * i, size, i);

        endtime = MPI_Wtime();
        std::cout << "Time " << endtime - starttime << " of process "
                  << world_rank << std::endl;
      }
    }
    free(rbuf);
  }
}

void broadcast(size_t size) {
  // Get the number of processes
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  // double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *rbuf = (char *)calloc(size, sizeof(char));

  double starttime, endtime;
  starttime = MPI_Wtime();
  if (world_rank == 0) {
    char *sbuf = (char *)malloc(size);
    for (size_t i = 0; i < size; i++) {
      sbuf[i] = (char)rand() % 256;
    }
    for (int i = 0; i < world_size; i++) {
      if (i != world_rank) {
        MPI_Send(sbuf, size, MPI_CHAR, i, 0, MPI_COMM_WORLD);
      }
    }
    free(sbuf);
  } else {
    MPI_Recv(rbuf, size, MPI_CHAR, 0, 0, MPI_COMM_WORLD, MPI_STATUS_IGNORE);
    printf("Process %d received token size %lu from %d \n", world_rank, size, 0);
    free(rbuf);
    endtime = MPI_Wtime();
    std::cout << "Time " << endtime - starttime << " of process " << world_rank
              << std::endl;
  }
}

void ring(size_t size) {
  // Get the number of processes
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *token = (char *)calloc(size, sizeof(char));
  if (world_rank != 0) {
        MPI_Recv(token, size, MPI_CHAR, world_rank - 1, 0, MPI_COMM_WORLD,
             MPI_STATUS_IGNORE);
    printf("Process %d received token size %lu from process %d\n", world_rank,
           size, world_rank - 1);
    printf("----------------------------------\n");
  } else {
    for (size_t i = 0; i < size; i++) {
      token[i] = (char)rand() % 256;
    }
    starttime = MPI_Wtime();
  }
  printf("Process %d send token size %lu \n", world_rank, size);
  MPI_Send(token, size, MPI_CHAR, (world_rank + 1) % world_size, 0,
           MPI_COMM_WORLD);
  // Now process 0 can receive from the last process.
  if (world_rank == 0) {
        MPI_Recv(token, size, MPI_INT, world_size - 1, 0, MPI_COMM_WORLD,
             MPI_STATUS_IGNORE);
    printf("Process %d received token size %lu from process %d\n", world_rank,
           size, world_size - 1);
    printf("----------------------------------\n");
    endtime = MPI_Wtime();
    printf("That took %f seconds\n", endtime - starttime);
    std::cout << "Time " << endtime - starttime << std::endl;
    free(token);
  }
}
