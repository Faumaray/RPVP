#include "lab_two.hpp"

const double e = 0.000001;
double drand(double low, double high) {
  return ((double)rand() * (high - low)) / (double)RAND_MAX + low;
}
void monte_carlo(double (*func)(double, double), double lower_x, double lower_y,
                 double upper_x, double upper_y, int n) {
  int world_size, world_rank;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  double starttime, endtime;
  starttime = MPI_Wtime();

  srand(world_rank);
  int in = 0;
  double s = 0;
  for (int i = world_rank; i < n; i += world_size) {
    double x = drand(lower_x, upper_x);
    double y = drand(lower_y, upper_y);
    in++;
    double tmp = func(x, y);
    if (tmp == -1) {
      continue;
    }
    s += tmp;
  }
  int gin = 0;
  MPI_Reduce(&in, &gin, 1, MPI_INT, MPI_SUM, 0, MPI_COMM_WORLD);
  double gsum = 0.0;
  MPI_Reduce(&s, &gsum, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

  if (world_rank == 0) {
    double v = 1;
    if (upper_x != 0 && gin != 0) {
      v = upper_x * gin / n;
    }
    double res = v * gsum / gin;
    printf("Result: %.12f, n %d\n", res, n);
    endtime = MPI_Wtime();
    printf("Time estimated: %f", endtime - starttime);
  }
}

void monte_carlo(double (*func)(double, double), double lower_x, double lower_y,
                 double upper_x, double upper_y, double difference, bool append,
                 BoundDifference bound, int n) {
  int world_size, world_rank;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  double starttime, endtime;
  starttime = MPI_Wtime();

  srand(world_rank);
  int in = 0;
  double s = 0;
  for (int i = world_rank; i < n; i += world_size) {
    double x;
    double y;
    switch (bound) {
    case BoundDifference::lower_x:
      y = drand(lower_y, upper_y);
      lower_x = append ? y + difference : difference - y;
      x = drand(lower_x, upper_x);
      break;
    case BoundDifference::lower_y:
      x = drand(lower_y, upper_y);
      lower_y = append ? x + difference : difference - x;
      y = drand(lower_x, upper_x);
      break;
    case BoundDifference::upper_x:
      y = drand(lower_y, upper_y);
      upper_x = append ? y + difference : difference - y;
      x = drand(lower_x, upper_x);
      break;
    case BoundDifference::upper_y:
      x = drand(lower_y, upper_y);
      upper_y = append ? x + difference : difference - x;
      y = drand(lower_x, upper_x);
      break;
    case BoundDifference::lower_x_against:
      y = drand(lower_y, upper_y);
      lower_x = append ? y + difference : y - difference;
      x = drand(lower_x, upper_x);
      break;
    case BoundDifference::lower_y_against:
      x = drand(lower_y, upper_y);
      lower_y = append ? x + difference : x - difference;
      y = drand(lower_x, upper_x);
      break;
    case BoundDifference::upper_x_against:
      y = drand(lower_y, upper_y);
      upper_x = append ? y + difference : y - difference;
      x = drand(lower_x, upper_x);
      break;
    case BoundDifference::upper_y_against:
      x = drand(lower_y, upper_y);
      upper_y = append ? x + difference : x - difference;
      y = drand(lower_x, upper_x);
      break;
    };
    in++;
    double tmp = func(x, y);
    if (tmp == -1) {
      continue;
    }
    s += tmp;
  }
  int gin = 0;
  MPI_Reduce(&in, &gin, 1, MPI_INT, MPI_SUM, 0, MPI_COMM_WORLD);
  double gsum = 0.0;
  MPI_Reduce(&s, &gsum, 1, MPI_DOUBLE, MPI_SUM, 0, MPI_COMM_WORLD);

  if (world_rank == 0) {
    double v = 1;
    if (upper_x != 0 && gin != 0) {
      v = upper_x * gin / n;
    }
    double res = v * gsum / gin;
    printf("Result: %.12f, n %d\n", res, n);
    endtime = MPI_Wtime();
    printf("Time estimated: %f", endtime - starttime);
  }
}

void midpoint_rule(double (*func)(double), double a, double b) {
  int world_size, world_rank;
  MPI_Comm_size(MPI_COMM_WORLD, &world_size);
  MPI_Comm_rank(MPI_COMM_WORLD, &world_rank);
  double starttime, endtime;
  starttime = MPI_Wtime();
  int n0 = 1;
  int n = n0, k;
  double sq[2], delta = 1;
  for (k = 0; delta > e; n *= 2, k ^= 1) {
    int points_per_proc = n / world_size;
    int lb = world_rank * points_per_proc;
    int ub =
        (world_rank == world_size - 1) ? (n - 1) : (lb + points_per_proc - 1);
    double h = (b - a) / n;
    double s = 0.0;
    for (int i = lb; i <= ub; i++) {
      s += func(a + h * (i + 0.5));
    }
    MPI_Allreduce(&s, &sq[k], 1, MPI_DOUBLE, MPI_SUM, MPI_COMM_WORLD);
    sq[k] *= h;
    if (n > n0) {
      delta = fabs(sq[k] - sq[k ^ 1]) / 3.0;
    }
  }
  if (world_rank == 0) {
    printf("Result Pi: %.12f; Runge rule: EPS %e, n %d\n", sq[k] * sq[k], e,
           n / 2);
    endtime = MPI_Wtime();
    printf("Time estimated: %f\n", endtime - starttime);
  }
}
