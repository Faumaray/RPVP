#pragma once
#include <mpi.h>
#include <vector>

template <typename T> class matrix {
public:
  static_assert(std::is_arithmetic<T>::value::type, "Type must be arithmetic");
  std::vector<std::vector<T>> value;
  template <
      typename = typename std::enable_if<std::is_arithmetic<T>::value, T>::type>
  std::vector<T> decompose();
  template <
      typename = typename std::enable_if<std::is_arithmetic<T>::value, T>::type>
  void sgemv(std::vector<T> row, std::vector<T> miltiplication_vector,
             std::vector<T> result_vector);
};
