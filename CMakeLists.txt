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
set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -Wl,-Bstatic,--whole-archive -lwinpthread -Wl,--no-whole-archive")
endif()

add_library(lab_one SHARED src/lab_one.cpp)
target_link_libraries(lab_one PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++)
add_library(lab_two SHARED src/lab_two.cpp)
target_link_libraries(lab_two PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++)
add_executable(mpi src/main.cpp)



# MPI
target_link_libraries(mpi PUBLIC lab_one -static-libgcc -static-libstdc++)
target_link_libraries(mpi PUBLIC lab_two -static-libgcc -static-libstdc++)
target_link_libraries(mpi PUBLIC MPI::MPI_CXX -static-libgcc -static-libstdc++)
install(CODE [[
  file(GET_RUNTIME_DEPENDENCIES
    EXECUTABLES ${CMAKE_CURRENT_SOURCE_DIR}/target/bin/mpi
    RESOLVED_DEPENDENCIES_VAR _r_deps
    UNRESOLVED_DEPENDENCIES_VAR _u_deps
    PRE_INCLUDE_REGEXES "^[Mm][Yy][Ll][Ii][Bb][Rr][Aa][Rr][Yy]\\.[Dd][Ll][Ll]$"
  )
  foreach(_file ${_r_deps})
    file(INSTALL
      DESTINATION ${CMAKE_CURRENT_SOURCE_DIR}/target/lib
      TYPE SHARED_LIBRARY
      FOLLOW_SYMLINK_CHAIN
      FILES "${_file}"
    )
  endforeach()
  list(LENGTH _u_deps _u_length)
  if("${_u_length}" GREATER 0)
    message(WARNING "Unresolved dependencies detected!")
  endif()
]])