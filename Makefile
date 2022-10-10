CC=mpic++
CFLAGS=-rdynamic -g -fPIC -O2 -s -DNDEBUG -Iinclude
EXECUTABLE=mpi
args=1 r 1024
build: prepare liblab_one.so liblab_two.so liblab_three.so
	$(CC) -O2 -s -DNDEBUG -lm -Iinclude -Wl,-rpath,$(PWD)/target -L./target -l:liblab_one.so -l:liblab_two.so -l:liblab_three.so -o target/$(EXECUTABLE) src/main.cpp

liblab_one.so: lab_one.o
	$(CC) -shared -export-dynamic -o target/$@ build/$<

liblab_two.so: lab_two.o
	$(CC) -shared -export-dynamic -o target/$@ build/$<

liblab_three.so: lab_three.o
	$(CC) -shared -export-dynamic -o target/$@ build/$<

lab_one.o:
	$(CC) $(CFLAGS) -c src/lab_one.cpp -o build/$@

lab_two.o:
	$(CC) $(CFLAGS) -c src/lab_two.cpp -o build/$@

lab_three.o:
	$(CC) $(CFLAGS) -c src/lab_three.cpp -o build/$@

run:
	mpirun -np 2 target/$(EXECUTABLE) $(args)


prepare:
	mkdir -p target
	mkdir -p build


clean:
	rm -rf target
	rm -rf build
