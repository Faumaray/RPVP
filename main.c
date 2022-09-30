#include "lab_one.h"

#define size 1024

int main(int argc, char **argv) {

  void (*mode)(size_t);
  size_t optind;
  enum { b, kb, mb } count = kb;
  for (optind = 1; optind < argc && argv[optind][0] == '-'; optind++) {
    switch (argv[optind][1]) {
    case 'r':
      printf("RING TEST MODE\n");
      mode = &ring;
      break;
    case 'b':
      printf("BROADCAST TEST MODE\n");
      mode = &broadcast;
      break;
    case 'g':
      printf("GATHER TEST MODE\n");
      mode = &gather;
      break;
    case 'a':
      printf("ALL TO ALL TEST MODE\n");
      mode = &alltoall;
      break;
    default:
      fprintf(stderr, "Usage: %s [-rbga] [-b kb mb]\n", argv[0]);
      exit(EXIT_FAILURE);
    }
  }
  // Initialize the MPI environment
  MPI_Init(NULL, NULL);

  mode(size);

  MPI_Finalize();
}
