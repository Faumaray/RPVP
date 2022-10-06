#include "lab_two.hpp"

const int n = 10000000;
const double e = 0.000001;
double drand(double low, double high) {
    return ((double)rand() * (high - low)) / (double)RAND_MAX + low;
}
void monte_carlo(double (*func)(double, double), double lower_x, double lower_y,
                 double upper_x, double upper_y) {
    int world_size = MPI::COMM_WORLD.Get_size();
    int world_rank = MPI::COMM_WORLD.Get_rank();
    double starttime, endtime;
    starttime = MPI::Wtime();

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
    MPI::COMM_WORLD.Reduce(&in, &gin, 1, MPI::INT, MPI::SUM, 0);
    double gsum = 0.0;
    MPI::COMM_WORLD.Reduce(&s, &gsum, 1, MPI::DOUBLE, MPI::SUM, 0);

    if (world_rank == 0) {
        double v = upper_x * gin / n;
        double res = v * gsum / gin;
        printf("Result: %.12f, n %d\n", res, n);
        endtime = MPI::Wtime();
        printf("Time estimated: %f", endtime - starttime);
    }
}

void midpoint_rule(double (*func)(double), double a, double b) {
    int world_size = MPI::COMM_WORLD.Get_size();
    int world_rank = MPI::COMM_WORLD.Get_rank();
    double starttime, endtime;
    starttime = MPI::Wtime();
    int n0 = 1;
    int n = n0, k;
    double sq[2], delta = 0;
    for (k = 0; delta < e; n *= 2, k ^= 1) {
        int points_per_proc = n / world_size;
        int lb = world_rank * points_per_proc;
        int ub = (world_rank == world_size - 1) ? (n - 1)
                                                : (lb + points_per_proc - 1);
        double h = (b - a) / n;
        double s = 0.0;
        for (int i = lb; i <= ub; i++) {
            s += func(a + h * (i + 0.5));
        }
        MPI::COMM_WORLD.Allreduce(&s, &sq[k], 1, MPI::DOUBLE, MPI::SUM);
        sq[k] *= h;
        if (n > n0) {
            delta = fabs(sq[k] - sq[k ^ 1]) / 3.0;
        }
    }
    if (world_rank == 0) {
        printf("Result Pi: %.12f; Runge rule: EPS %e, n %d\n", sq[k] * sq[k], e,
               n / 2);
        endtime = MPI::Wtime();
        printf("Time estimated: %f\n", endtime - starttime);
    }
}
