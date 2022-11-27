cmake_minimum_required(VERSION 3.9.1)
project(mpi)

set(CMAKE_CXX_STANDARD 11)
set(CXX mpicxx)
set(CC mpicc)
find_package(MPI REQUIRED)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/bin)
if(MINGW)
set(CMAKE_CXX_STANDARD_LIBRARIES "-static-libgcc -static-libstdc++")
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic")
endif()


add_library(MINGW_WINPTHREAD_LIBRARY SHARED IMPORTED)
add_library(lab_one SHARED src/lab_one.cpp)
target_link_libraries(lab_one PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic)
add_library(lab_two SHARED src/lab_two.cpp)
target_link_libraries(lab_two PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic)
add_executable(mpi src/main.cpp)


# MPI
target_link_libraries(mpi PUBLIC lab_one -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic)
target_link_libraries(mpi PUBLIC lab_two -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic)
target_link_libraries(mpi PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++ -Wl,-Bstatic -lstdc++ -lpthread -Wl,-Bdynamic)