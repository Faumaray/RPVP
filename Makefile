CC=mpicc
CFLAGS=-std=c99 -rdynamic -g -fPIC
EXECUTABLE=mpi
args=-r

all: build run

run: build
	mpirun -np 2 $(EXECUTABLE) $(args)

build: liblab_one.so liblab_two.so liblab_three.so
	$(CC) -std=c99 -Wl,-rpath,$(PWD) -L. -l:$< -o $(EXECUTABLE) main.c

liblab_one.so: lab_one.o
	$(CC) -shared -export-dynamic -o $@ $<

liblab_two.so: lab_two.o
	$(CC) -shared -export-dynamic -o $@ $<

liblab_three.so: lab_three.o
	$(CC) -shared -export-dynamic -o $@ $<

lab_one.o:
	$(CC) $(CFLAGS) -c lab_one.c

lab_two.o:
	$(CC) $(CFLAGS) -c lab_two.c

lab_three.o:
	$(CC) $(CFLAGS) -c lab_three.c

clean:
	rm -rf *.o *.so mpi
