# How to Build

## If has 'Make'
```sh
make build
```
## If no make

```sh
mpicc -std=c99 -rdynamic -g -fPIC -c lab_one.c
mpicc -std=c99 -rdynamic -g -fPIC -c lab_two.c
mpicc -std=c99 -rdynamic -g -fPIC -c lab_three.c
mpicc shared -export-dynamic -o liblab_one.so lab_one.o
mpicc shared -export-dynamic -o liblab_two.so lab_two.o
mpicc shared -export-dynamic -o liblab_three.so lab_three.o
mpicc -lm -std=c99 -Wl,-rpath,$(PWD) -L. -l:liblab_one.so -l:liblab_two.so -l:liblab_three.so -o mpi main.c
```
