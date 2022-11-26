#pragma once
#include <mpi.h>
#include <vector>
#include <iostream>

void SGEMV(int dim, double *matrix_data, double *vector_data, double *result);
