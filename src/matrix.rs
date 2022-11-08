use mpi::traits::*;

#[derive(Equivalence)]
pub struct Matrix<T>(Vec<Vec<T>>)
where
    T: num::traits::Num;

impl<T> Matrix<T> where T: num::traits::Num {}
