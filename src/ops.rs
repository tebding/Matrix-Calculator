//matrix_ops.rs
//contains functions to execute math operations on matrices



//adds the values of each matrix's equivalent indices together
pub fn matrix_add(matrix1: &mut Vec<f32>, matrix2: &mut Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(matrix1.len());
    while matrix1.is_empty() != true {
        res.push(matrix1.remove(0) + matrix2.remove(0));
    }
    res
}


//performs matrix multiplication
pub fn matrix_multiply(matrix1: &Vec<f32>, size1: &Vec<usize>,
                       matrix2: &Vec<f32>, mut size2: &mut Vec<usize>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    
    //by transposing the second matrix, the abstraction requires fewer translations
    let matrix2 = matrix_transpose(&matrix2, &mut size2);
    let cols = size1[1];
    let rows1 = size1[0];
    let rows2 = size2[0];
    let mut temp: f32 = 0.0;
    
    for i in 0..rows1 { //iterate through each row of matrix1
        let row1 = i * cols;
        for j in 0..rows2 { //iterate through each row of matrix2(transposed)
            let row2 = j * cols;
            for this_col in 0..cols {
                temp += matrix1[row1+this_col] * matrix2[row2+this_col];
            }
            res.push(temp);
            temp = 0.0;
        }
    }
    res
}


//adds the given value to each index in the matrix
pub fn matrix_scalar_add(matrix: &mut Vec<f32>, scalar: f32) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(matrix.len());
    while matrix.is_empty() != true {
        res.push(matrix.remove(0) + scalar);
    }
    res
}


//multiplies each index of the matrix by a scalar value
pub fn matrix_scalar_multiply(matrix: &mut Vec<f32>, scalar: f32) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(matrix.len());
    while matrix.is_empty() != true {
        res.push(matrix.remove(0) * scalar);
    }
    res
}


//calculates the Determinant of the given matrix
  //the output will be a vector of length 1 (in order for print methodology to be consistent)
pub fn matrix_determinant(matrix: &Vec<f32>, size: &Vec<usize>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(1);
    if size[0] == 2 { //at size 2, determinant is the following
        let det = (matrix[0] * matrix[3]) - (matrix[1] * matrix[2]);
        res.push(det);
        res
    }
    else if size[0] > 2 {
        let cols = size[1];
        let mut det: f32 = 0.0;
        for i in 0..cols {
            let row_col: Vec<usize> = vec![0, i];
            let matrix_minor = get_matrix_minor(&matrix, &size, row_col);
            let minor_size: Vec<usize> = vec![cols-1, cols-1, (cols-1)*(cols-1)];
            let tmp = matrix_determinant(&matrix_minor, &minor_size);

            let temp: f32 = tmp[0] * matrix[i as usize];
            if (i % 2) == 0 { //at even-numbered column
                det += temp;
            }
            else { //at odd-numbered column
                det -= temp;
            }
        }
        res.push(det);
        res
    }
    else { //will address/clean if a relevant error is found
        panic!("this should be unreachable, so ????"); 
    }
}


//calculates the Transpose of the given matrix
pub fn matrix_transpose(matrix: &Vec<f32>, size: &mut Vec<usize>) -> Vec<f32> {
    let rows = size.remove(0);
    let cols = size.remove(0);
    let capacity = size.remove(0);
    //the dimensions of a transposed matrix are swapped
    size.push(cols);
    size.push(rows);
    size.push(capacity);
    
    let mut res: Vec<f32> = Vec::with_capacity(capacity);
    for i in 0..cols {
        let mut k: usize = i;
        for _j in 0..rows {
            res.push(matrix[k]);
            k += cols;
        }
    }
    res
}


//calculates the Adjugate of the given matrix
pub fn matrix_adjugate(matrix: &Vec<f32>, mut size: &mut Vec<usize>) -> Vec<f32> {
    let rows = size[0];
    let cols = size[1];
    let mut res: Vec<f32> = Vec::with_capacity(size[2]);
    let mut to_neg = false;
    
    for i in 0..rows {
        for j in 0..cols {
            let row_col: Vec<usize> = vec![i, j];
            let minor = get_matrix_minor(&matrix, &size, row_col);
            let minor_size: Vec<usize> = vec![rows-1, cols-1, (rows-1)^2];
            let mut temp = matrix_determinant(&minor, &minor_size);
            if to_neg == true {
                temp[0] *= -1.0;
            }
            res.push(temp[0]);
            to_neg = !to_neg;
        }
        //if there are an even number of elements per side, then we must
        //offset the check for cofactoring to properly checkerboard the matrix
        if rows%2 == 0 {
            to_neg = !to_neg;
        }
    }
    res = matrix_transpose(&res, &mut size);
    res
}


//calculates the Inverse of the given matrix
pub fn matrix_inverse(matrix: &Vec<f32>, mut size: &mut Vec<usize>) -> Vec<f32> {
    let det = matrix_determinant(&matrix, &size);
    let inv_det = 1.0 / (det[0] as f32);
    let mut adj = matrix_adjugate(&matrix, &mut size);
    let res = matrix_scalar_multiply(&mut adj, inv_det);
    res
}


#[cfg(test)]
mod op_tests {
    use super::*;
    #[test]
    fn add_test() {
        
    }
    
    #[test]
    fn multiply_test() {
        
    }
    
    #[test]
    fn scalar_add_test() {
        
    }
    
    #[test]
    fn scalar_multiply_test() {
        
    }
    
    #[test]
    fn determinant_test() {
        
    }
    
    #[test]
    fn transpose_test() {
        
    }
    #[test]
    fn adjugate_test() {
        
    }
    #[test]
    fn inverse_test() {
        
    }
}
