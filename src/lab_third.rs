pub use num_traits::Num;
pub type Matrix<T> = Vec<Vec<T>>;

pub trait LabThree<T>
where
    T: Num + Copy + Default + std::clone::Clone + std::ops::AddAssign + num::FromPrimitive,
    rand::distributions::Standard: rand::distributions::Distribution<T>,
    Vec<T>: mpi::traits::Buffer
        + mpi::datatype::Pointer
        + mpi::traits::AsDatatype
        + mpi::traits::BufferMut,
{
    fn generate_test_data(rows: usize, columns: usize, randomize: bool) -> (Vec<T>, Vec<T>) {
        use rand::prelude::*;
        let mut rand = rand::thread_rng();
        let mut matrix: Vec<T> = vec![T::default() + T::from_i32(1).unwrap(); columns * rows];
        let mut vector: Vec<T> = vec![T::default() + T::from_i32(2).unwrap(); columns];
        if randomize {
            for i in 0..rows * columns {
                let value: T = rand.gen();
                matrix[i] = value;
            }
            for i in 0..columns {
                let value: T = rand.gen();
                vector[i] = value;
            }
        }

        (matrix, vector)
    }
    fn get_distribution(&self, rows: usize, columns: usize) -> (Vec<i32>, Vec<i32>);
    fn sgemv(&self, generate: bool, rows: usize, columns: usize) -> Vec<T>;
}
