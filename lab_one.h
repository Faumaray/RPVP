#include <mpi.h>
#include <stdio.h>
#include <stdlib.h>

void ring(size_t size);
void broadcast(size_t size);
void gather(size_t size);
void alltoall(size_t size);
