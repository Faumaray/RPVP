cmake_minimum_required(VERSION 3.9.1)
project(mpi)

set(CMAKE_CXX_STANDARD 11)

find_package(MPI REQUIRED)

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/include)
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}/target/bin)


add_library(lab_one SHARED src/lab_one.cpp)
target_link_libraries(lab_one PUBLIC MPI::MPI_CXX)
add_library(lab_two SHARED src/lab_two.cpp)
target_link_libraries(lab_two PUBLIC MPI::MPI_CXX)
add_library(lab_three SHARED src/lab_three.cpp)
target_link_libraries(lab_three PUBLIC MPI::MPI_CXX)
add_executable(mpi src/main.cpp)

# MPI
target_link_libraries(mpi PUBLIC lab_one)
target_link_libraries(mpi PUBLIC lab_two)
target_link_libraries(mpi PUBLIC lab_three)
target_link_libraries(mpi PUBLIC MPI::MPI_CXX)