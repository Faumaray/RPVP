#include "lab_one.h"
void alltoall(size_t size) {
  // Get the number of processes
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  // double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *rbuf = calloc(size * world_size - 1, sizeof(char));
  double starttime, endtime;

  MPI_Status status[world_size];
  /* Request objects for non-blocking send and receive */
  MPI_Request send_req[world_size], recv_req[world_size];
  char *sbuf = malloc(size);
  for (int i = 0; i < size; i++) {
    sbuf[i] = (char)rand() % 256;
  }

  starttime = MPI_Wtime();
  for (int i = 0; i < world_size; i++) {
    MPI_Isend(sbuf, size, MPI_CHAR, i, 0, MPI_COMM_WORLD, &send_req[i]);
  }
  MPI_Waitall(world_size, send_req, status);
  for (int i = 0; i < world_size; i++) {
    MPI_Irecv(rbuf + (size * i), size, MPI_CHAR, i, 0, MPI_COMM_WORLD,
              &recv_req[i]);
  }
  MPI_Waitall(world_size, recv_req, status);

  endtime = MPI_Wtime();
  printf("Time %f of process %d\n", endtime - starttime, world_rank);
  free(sbuf);
  free(rbuf);
}

void gather(size_t size) {
  // Get the number of processes
  int world_size;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  // double starttime, endtime;
  // Get the rank of the process
  int world_rank;
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  char *rbuf = calloc(size * world_size - 1, sizeof(char));
  double starttime, endtime;
  starttime = MPI_Wtime();
  if (world_rank != 0) {
    char *sbuf = malloc(size);
    for (int i = 0; i < size; i++) {
      sbuf[i] = (char)rand() % 256;
    }
    MPI_Send(sbuf, size, MPI_CHAR, 0, 0, MPI_COMM_WORLD);
    free(sbuf);
  } else {
    for (int i = 1; i < world_size; i++) {
      if (i != world_rank) {
        MPI_Recv(rbuf + (size * i), size, MPI_CHAR, i, 0, MPI_COMM_WORLD,
                 MPI_STATUS_IGNORE);

        printf("Process %d received in element %lu data %hhd from %d \n",
               world_rank, size * i, rbuf[size * i], i);

        endtime = MPI_Wtime();
        printf("Time %f of process %d\n", endtime - starttime, world_rank);
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
  char *rbuf = calloc(size, sizeof(char));

  double starttime, endtime;
  starttime = MPI_Wtime();
  if (world_rank == 0) {
    char *sbuf = malloc(size);
    for (int i = 0; i < size; i++) {
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
    printf("Process %d received data(first element) %hhd from %d \n",
           world_rank, rbuf[0], 0);
    free(rbuf);
    endtime = MPI_Wtime();
    printf("Time %f of process %d\n", endtime - starttime, world_rank);
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
           sizeof(token), world_rank - 1);
    printf("----------------------------------\n");
  } else {
    for (int i = 0; i < size; i++) {
      token[i] = (char)rand() % 256;
    }
    starttime = MPI_Wtime();
  }
  printf("Process %d send token size %lu \n", world_rank, sizeof(token));
  MPI_Send(token, size, MPI_CHAR, (world_rank + 1) % world_size, 0,
           MPI_COMM_WORLD);
  // Now process 0 can receive from the last process.
  if (world_rank == 0) {
    MPI_Recv(token, size, MPI_INT, world_size - 1, 0, MPI_COMM_WORLD,
             MPI_STATUS_IGNORE);
    printf("Process %d received token size %lu from process %d\n", world_rank,
           sizeof(token), world_size - 1);
    printf("----------------------------------\n");
    endtime = MPI_Wtime();
    printf("That took %f seconds\n", endtime - starttime);
    free(token);
  }
}
