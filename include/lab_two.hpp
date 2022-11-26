#include <assert.h>
#include <math.h>
#include <mpicxx.h>
#include <stdio.h>
#include <stdlib.h>

enum BoundDifference {
    lower_x,
    lower_y,
    upper_x,
    upper_y,
    lower_x_against,
    lower_y_against,
    upper_x_against,
    upper_y_against
};

void monte_carlo(double (*func)(double, double), double lower_x, double lower_y,
                 double upper_x, double upper_y, int n);

void monte_carlo(double (*func)(double, double), double lower_x, double lower_y,
                 double upper_x, double upper_y, double difference, bool append,
                 BoundDifference bound, int n);
void midpoint_rule(double (*func)(double), double a, double b);

static inline double one_on_one(double x) {
    return (1 - exp(0.7 / x)) / (2 + x);
}
static inline double two_on_one(double x) {
    return log(1 + x) / x;
}
static inline double three_on_one(double x) {
    return (sqrt(x * (3 - x))) / (x + 1);
}
static inline double four_on_one(double x) {
    return sin(x + 2) / (0.4 + cos(x));
}
static inline double five_on_one(double x) {
    return x / (pow(sin(2 * x), 3));
}
static inline double six_on_one(double x) {
    return pow(x, 4) / (0.5 * pow(x, 2) + x + 6);
}

static inline double one_on_two(double x, double y) {
    if (x < 0 || x > 1 || y < 2 || y > 5) {
        return -1;
    }
    return x / pow(y, 2);
}
static inline double two_on_two(double x, double y) {
    if (x < 0 || x > 1 || y < 0 || y > (1 - x)) {
        return -1;
    }
    return pow(exp(x + y), 2);
}
static inline double three_on_two(double x, double y) {
    if (x < -1 || x > 0 || y < 0 || y > 1) {
        return -1;
    }
    return pow(exp(x - y), 2);
}
