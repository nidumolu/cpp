use ndarray::Array1; // Use the specific 1D alias
use ndarray::Array;

pub fn array_print() -> Array1<i32> {
    let arr = Array::from_vec(vec![1, 2, 3]);
    return arr;
}