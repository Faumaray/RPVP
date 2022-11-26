#include <lab_three.hpp>

void SGEMV(int dim, double *matrix_data, double *vector_data, double *result) {
  int size, rank;
  MPI_Comm_size(MPI_COMM_WORLD, &size);
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);
  double *localresult = new double[dim / size]{};
  double matrix[dim][dim]; // local matrix
  double timer = MPI_Wtime();
  MPI_Barrier(MPI_COMM_WORLD);
  MPI_Scatter(matrix_data, (dim * dim) / size, MPI_DOUBLE, matrix,
              (dim * dim) / size, MPI_DOUBLE, 0,
              MPI_COMM_WORLD); // Scatter the Matrix
  MPI_Bcast(vector_data, dim, MPI_DOUBLE, 0,
            MPI_COMM_WORLD); // Broadcast the Vector

  // Calculate the results
  for (int i = 0; i < (dim / size); i++)
    for (int j = 0; j < dim; j++)
      localresult[i] += vector_data[j] * matrix[i][j];
  MPI_Gather(localresult, (dim) / size, MPI_DOUBLE, result, (dim) / size,
             MPI_DOUBLE, 0, MPI_COMM_WORLD); // Gather the results
  timer = MPI_Wtime() - timer;
  std::cout << "Time Needed for all ops = " << timer << std::endl;
  if (rank == 0) {
    printf("Matrix  :\n");
    for (int i = 0; i < dim; i++) {
      for (int j = 0; j < dim; j++)
        std::cout << matrix_data[i + j] << std::endl;
    }
    printf("Vector :\n");
    for (int i = 0; i < dim; i++)
      printf("%.5f ", vector_data[i]);
    printf("\n\n");

    printf("Result :\n");
    for (int i = 0; i < dim; i++)
      printf("%.5f ", result[i]);
    printf("\n\n");
  }
}
