pub use num_traits::Num;
use rand::distributions::Standard;

pub trait LabSix<T>
where
    T: Num + Copy + Default + std::clone::Clone + std::ops::AddAssign + num::FromPrimitive,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    Vec<T>: mpi::traits::Buffer
        + mpi::datatype::Pointer
        + mpi::traits::AsDatatype
        + mpi::traits::BufferMut,
{
    fn vec_to_matrix(&self, vec: Vec<T>, rows: usize, columns: usize) -> Vec<Vec<T>> {
        let mut matrix = vec![vec![T::default(); columns]; rows];
        for i in 0..rows {
            for j in 0..columns {
                matrix[i][j] = vec[i * columns + j];
            }
        }
        matrix
    }

    fn generate_test_matrix_data(
        rows: usize,
        columns: usize,
        randomize: bool,
        rank: i32,
    ) -> (Vec<T>, Vec<T>) {
        use rand::prelude::*;
        if rank == 0 {
            if randomize {
                (
                    rand::thread_rng()
                        .sample_iter(Standard)
                        .take(rows * columns)
                        .collect(),
                    rand::thread_rng()
                        .sample_iter(Standard)
                        .take(columns * rows)
                        .collect(),
                )
            } else {
                (
                    vec![T::default() + T::from_i32(1).unwrap(); columns * rows],
                    vec![T::default() + T::from_i32(2).unwrap(); columns * rows],
                )
            }
        } else {
            (Vec::new(), vec![T::default(); columns])
        }
    }
    fn get_matrix_distribution(&self, rows: usize, columns: usize) -> Vec<i32>;
    #[allow(patterns_in_fns_without_body)]
    fn sgemm(&self, generate: bool, rows: usize, columns: usize, dims: usize);
}
