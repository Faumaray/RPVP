#include <cmath>
#include <fstream>
#include <math.h>
#include <mpi.h>
#include <stdio.h>
#include <time.h>

/*NUM_DIMS - размер декартовой топологии."кольцо" - одномерный тор*/
#define NUM_DIMS 1

const int N1 = 100, M1 = 100;
static double A[N1][M1], B[N1][M1], C[N1][N1];
int main(int argc, char *argv[]) {
  int rank, size, i, j, k, i1, j1, d, sour, dest;
  int N, M;
  int dims[NUM_DIMS], periods[NUM_DIMS], new_coords[NUM_DIMS];
  int reorder = 0;
  MPI_Comm comm_cart;
  MPI_Status st;
  double time1, time2;
  double *Avector, *Bvector, *Cvector;
  int dt1;
  MPI_Init(&argc, &argv);
  MPI_Comm_size(MPI_COMM_WORLD, &size);
  MPI_Comm_rank(MPI_COMM_WORLD, &rank);
  if (rank == 0) {
    /*Читаем матрицу A из файла A.txt, расположенного в папке с исполняемым
    файлом нулевого компьютера N и M располочены там же*/
    std::fstream f;
    f.open(argv[1], std::ios::in);
    f >> N >> M;
    /* Передаем N и M всем комьютерам */
    for (i = 1; i < size; i++) {
      MPI_Send(&N, 1, MPI_INT, i, 1, MPI_COMM_WORLD);
      MPI_Send(&M, 1, MPI_INT, i, 2, MPI_COMM_WORLD);
    }
    for (i = 0; i < N; i++) {
      for (j = 0; j < M; j++) {
        f >> A[i][j];
      }
    }
    f.close();
    /*Матрицу B читаем из файла B.txt, расположенного
    там же*/
    f.open(argv[2], std::ios::in);
    for (i = 0; i < M; i++) {
      for (j = 0; j < N; j++) {
        f >> B[j][i]; // Матрицу B транспонируем для удобства
      }
    }
    f.close();
  }
  if (rank != 0) {
    // Каждый процесс получает от нулевого N и M
    MPI_Recv(&N, 1, MPI_INT, 0, 1, MPI_COMM_WORLD, &st);
    MPI_Recv(&M, 1, MPI_INT, 0, 2, MPI_COMM_WORLD, &st);
  }
  // if (N % size != 0) {
  //   /*Если строки матрицы A нельзя поровну поделить по процессам в группе,
  //   нулевой выводит сообщение об ошибке*/
  //   if (rank == 0) {
  //     printf("Ошибка! Строк: %d, компьютеров:%d.\nНужно, чтобы строки матрицы
  //     "
  //            "A и столбцы матрицы B поровну делились по компьютерам!!!\n",
  //            N, size);
  //     fflush(stdout);
  //   }
  //   // Все процессы ждут, пока нулевой выведет сообщение об ошибке
  //   MPI_Barrier(MPI_COMM_WORLD);
  //   // Все задачи аварийно завершают работу
  //   MPI_Abort(MPI_COMM_WORLD, MPI_ERR_OTHER);
  //   return -1;
  // }
  int NS = 0;
  if (N % size == 0) {
    N = N / size;
  } else {
    if (rank == 0) {
      std::cout << N % size << std::endl;
      N = std::round((N / size)) + (N % size);
      NS = std::round((N / size));
    } else {
      N = std::round((N / size));
    }
  }
  std::cout << rank << " " << N << std::endl;
  if (rank == 0) {
    Avector = new double[NS * M];
    Bvector = new double[NS * M];
    /* В цикле нулевой процесс передает всем остальным процессам их полосы
    матриц A и B Для этого преобразуем матрицы в векторы, чтобы удобнее было
    передавать*/
    for (i = 1; i < size; i++) {
      for (i1 = 0; i1 < NS; i1++) {
        for (j1 = 0; j1 < M; j1++) {
          Avector[i1 * M + j1] = A[NS * i + i1][j1];
        }
      }
      for (i1 = 0; i1 < NS; i1++) {
        for (j1 = 0; j1 < M; j1++) {
          Bvector[i1 * M + j1] = B[i * NS + i1][j1];
        }
      }
      MPI_Send(Avector, NS * M, MPI_DOUBLE, i, 3, MPI_COMM_WORLD);
      MPI_Send(Bvector, NS * M, MPI_DOUBLE, i, 4, MPI_COMM_WORLD);
    }
    /* Свою полосу матрицы B нулевой процесс тожепреобразует в вектор */
    Bvector = new double[N * M];
    Avector = new double[N * M];
    for (i1 = 0; i1 < N; i1++) {
      for (j1 = 0; j1 < M; j1++) {
        Bvector[i1 * M + j1] = B[i1][j1];
      }
    }
  } else {
    Avector = new double[N * M];
    Bvector = new double[N * M];
    std::cout << "[" << rank << "]"
              << "N= " << N << std::endl;
    // Все ненулевые процессы получают от нулевого матрицы A и B
    MPI_Recv(Avector, N * M, MPI_DOUBLE, 0, 3, MPI_COMM_WORLD, &st);
    MPI_Recv(Bvector, N * M, MPI_DOUBLE, 0, 4, MPI_COMM_WORLD, &st);
    // и преобразуют матрицу А из вектора в матрицу
    for (i = 0; i < N; i++) {
      for (j = 0; j < M; j++) {
        A[i][j] = Avector[i * M + j];
      }
    }
  }
  // Каждый процесс выводит свои полосы для проверки
  fflush(stdout);
  printf("[%d]Матрица A:\n", rank);
  for (i = 0; i < N; i++) {
    for (j = 0; j < M; j++) {
      printf("%f ", A[i][j]);
    }
    printf("\n");
  }
  printf("[%d]Матрица B:\n", rank);
  for (i = 0; i < M; i++) {
    for (j = 0; j < N; j++) {
      printf("%f ", Bvector[j * M + i]);
    }
    printf("\n");
  }
  fflush(stdout);
  /* Обнуляем массив dims и заполняем массив periods для топологии "кольцо"*/
  for (i = 0; i < NUM_DIMS; i++) {
    dims[i] = 0;
    periods[i] = 1;
  }
  /*заполняем массив dims, где указываются размеры (одномерной) решетки */
  MPI_Dims_create(size, NUM_DIMS, dims);
  /* Создаем топологию "кольцо" с communicator(ом) comm_cart*/
  MPI_Cart_create(MPI_COMM_WORLD, NUM_DIMS, dims, periods, reorder, &comm_cart);
  /* Отображаем ранги на координаты компьютеров, с целью оптимизации отображения
  заданной виртуальной топологии на физическую топологию системы.
  */
  MPI_Cart_coords(comm_cart, rank, NUM_DIMS, new_coords);
  /* Каждая ветвь находит своих соседей вдоль кольца, в направлении меньших
   * значений рангов */
  MPI_Cart_shift(comm_cart, 0, -1, &sour, &dest);
  /* Засекаем начало умножения матриц */
  time1 = MPI_Wtime();
  /*Каждая ветвь производит умножение своих полос матриц */
  /* Самый внешний цикл for(k) - цикл по компьютерам */
  for (k = 0; k < size; k++) {
    // Читаем Матрицу B из буфера Bvector
    for (i = 0; i < N; i++) {
      for (j = 0; j < M; j++) {
        B[i][j] = Bvector[i * M + j];
      }
    }
    /*Каждая ветвь вычисляет координаты(вдоль строки) для результирующих
    элементов матрицы C, которые зависят от номера цикла k и ранга компьютера.
  */
    d = ((rank + k) % size) * N;
    /*Каждая ветвь производит умножение своей полосы матрицы A на текущую полосу
     * матрицы B */
    for (i = 0; i < N; i++) {
      for (j = 0; j < N; j++) {
        for (i1 = 0; i1 < M; i1++) {
          C[i][j + d] += A[i][i1] * B[j][i1];
        }
      }
    }
    /* Умножение полосы строк матрицы A на полосу столбцов матрицы B в каждой
     * ветви завершено */
    /* Каждая ветвь передает своим соседним ветвям с меньшим рангом вертикальные
    полосы матрицы B. Т.е. полосы матрицы B сдвигаются вдоль кольца компьютеров
  */
    MPI_Sendrecv_replace(Bvector, N * M, MPI_DOUBLE, dest, 12, sour, 12,
                         comm_cart, &st);
  }
  /* Умножение завершено. Каждая ветвь умножила свою полосу строк матрицы A на
  все полосы столбцов матрицы B. Засекаем время и выводим результат */
  printf("[%d] Time=%f\n", rank, (MPI_Wtime() - time1));
  /* Все ветви завершают системные процессы, связанные с топологией comm_cart */
  MPI_Comm_free(&comm_cart);
  /* Собираем матрицу C в процессе с рангом 0*/
  if (rank != 0) {
    Cvector = new double[N * N * size];
    for (i = 0; i < N; i++) {
      for (j = 0; j < N * size; j++) {
        Cvector[i * N * size + j] = C[i][j];
      }
    }
    MPI_Send(Cvector, N * N * size, MPI_DOUBLE, 0, 5, MPI_COMM_WORLD);
  } else {
    Cvector = new double[N * N * size];
    for (k = 1; k < size; k++) {
      MPI_Recv(Cvector, N * N * size, MPI_DOUBLE, k, 5, MPI_COMM_WORLD, &st);
      for (i = 0; i < N; i++) {
        for (j = 0; j < N * size; j++) {
          C[i + N * k][j] = Cvector[i * N * size + j];
        }
      }
    }
    // Матрица собрана: выводим результат
    printf("[%d]Матрица C:\n", rank);
    for (i = 0; i < N * size; i++) {
      for (j = 0; j < N * size; j++) {
        printf("%f ", C[i][j]);
      }
      printf("\n");
    }
  }
  // delete[]Avector;
  // delete[]Bvector;
  // delete[]Cvector;
  MPI_Finalize();
  return 0;
}
