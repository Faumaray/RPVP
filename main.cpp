#include "lab_one.hpp"
#include "lab_two.hpp"

// Lab 1 = (i%6)+1;
// Lab 2 = (i%3)+1;
int main(int argc, char **argv) {

    // // Initialize the MPI environment
    MPI::Init();
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
        break;
    } //
    // mode(size);
    //
    // midpoint_rule(five_on_one, 0.1, 0.5);
    // monte_carlo(two_on_two, 0, 0, 1, 1);
    MPI::Finalize();
    return 0;
}
