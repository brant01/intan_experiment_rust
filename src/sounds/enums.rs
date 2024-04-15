
use ndarray::{Array1, Array2};


pub enum DataArray {
    ArrayOne(Array1<f64>),
    ArrayTwo(Array2<f64>),
}