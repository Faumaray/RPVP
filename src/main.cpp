#include "lab_one.hpp"
#include "lab_two.hpp"
#include <cstdlib>

// Lab 1 = (i%6)+1;
// Lab 2 = (i%3)+1;
int main(int argc, char **argv) {

  // // Initialize the MPI environment
  MPI_Init(NULL, NULL);
  char **rng;
  void (*mode)(size_t);
  unsigned int lab;
  unsigned int count;
  unsigned int variant;
  switch (strtoul(argv[1], rng, 10)) {
  case 1:
    count = strtoul(argv[3], rng, 10);
    switch (*argv[2]) {
    case 'r':
      printf("RING TEST MODE\n");
      ring(count);
      break;
    case 'b':
      printf("BROADCAST TEST MODE\n");
      broadcast(count);
      break;
    case 'g':
      printf("GATHER TEST MODE\n");
      gather(count);
      break;
    case 'a':
      printf("ALL TO ALL TEST MODE\n");
      alltoall(count);
      break;
    }
  case 2:
    variant = count = strtoul(argv[3], rng, 10);
    switch (*argv[2]) {
    case 'm':
      switch ((variant % 6) + 1) {
      case 1:
        midpoint_rule(one_on_one, 1, 2);
        break;
      case 2:
        midpoint_rule(two_on_one, 0.1, 1);
        break;
      case 3:
        midpoint_rule(three_on_one, 1, 1.2);
        break;
      case 4:
        midpoint_rule(four_on_one, -1, 1);
        break;
      case 5:
        midpoint_rule(five_on_one, 0.1, 0.5);
        break;
      case 6:
        midpoint_rule(six_on_one, 0.4, 1.5);
        break;
      }
      break;
    case 'c':
      int n = strtol(argv[4], rng, 10);
      switch ((variant % 3) + 1) {
      case 1:
        monte_carlo(one_on_two, 0.0l, 2.0l, 1.0l, 5.0l, n);
        break;
      case 2:
        monte_carlo(two_on_two, 0.0l, 0, 1, 0, 1, false,
                    BoundDifference::upper_y, n);
        break;
      case 3:
        monte_carlo(three_on_two, -1, 0, 0, 1, n);
        break;
      }
      break;
    }
  } //
  MPI_Finalize();
  return 0;
}
