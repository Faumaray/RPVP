CC=mpic++
CFLAGS=-std=c++0x -g -fPIC -O2 -s -DNDEBUG -Iinclude
EXECUTABLE=mpi
args=1 r 1024
proc=2

help_ru: ## Prints help for targets with comments
	$(info Комманда для сборки: make build)
	$(info Комманда для запуска: make run proc={кол-во процессов} args={об этом далее})
	$(info Args=(номер лабораторной) (для первой лабы: первая буква метода [r\b\g\a]) (размер буфера))
	$(info Args=(номер лабораторной) (для второй лабы: первая буква метода [m\c]) (последняя цифра зачётки) (размер буфера))
	$(info Все аргументы писать без скобок)

help_en: ## Prints help for targets with comments
	$(info For build run: make build)
	$(info For run the program run: make run proc={num of process} args={your args})
	$(info Args=(number of lab) (for first lab: first letter of the method [r\b\g\a]) (size of the buffer))
	$(info Args=(number of lab) (for second lab: first name of the method [m\c]) (last number of the record book) (size of the buffer))

cmake: prepare
	cmake -B ./build -G Ninja -Wno-dev
	cmake --build build
	cmake --install build

build: prepare liblab_one.so liblab_two.so liblab_three.so
	$(CC) -std=c++0x -O2 -s -DNDEBUG -lm -Iinclude -Wl,-rpath,$(PWD)/target -L./target -l:liblab_one.so -l:liblab_two.so -l:liblab_three.so -o target/$(EXECUTABLE) src/main.cpp

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
	mpirun -np $(proc) target/$(EXECUTABLE) $(args)


prepare:
	mkdir -p target
	mkdir -p build


clean:
	rm -rf target
	rm -rf build
