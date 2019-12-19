//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.

use matrix_calculator::{*};
use std::io::{self, Write};

fn main() {
    println!("Matrix calculator to operate on 3x3 matrices\n");
    
    loop { //calculator passively runs here
        let res = evaluate();
        println!("Result:");
        match res {
            Ok(res) => {
                if res.len() == 1 { //since determinant returns single value
                    println!("{}\n", res[0]);
                }
                else {
                    //matrix_print(&res);
                }
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    
    
}

/*
   called in main().
   executes the input, setup, and calculations.
   returns the resultant matrix to main().
*/
fn evaluate() -> Result<Vec<f32>, String> {
    print!("Enter the operation to be performed (CTRL+C to exit). Valid operations include:\n\
            Add, Scalar Add, Multiply, Scalar Multiply, Determinant, Transpose, Inverse, Adjugate\n\
            > ");
    io::stdout().flush().expect("failed to flush");
    
    let mut oper = String::new();
    io::stdin().read_line(&mut oper)
    .expect("failed to read line");
    
    //the matrix to operate on. For scalar/binary ops, other vars assigned later
    let mut matrix: Vec<f32> = Vec::new();
    let mut result: Vec<f32> = Vec::new();
    let mut matrix_size = set_matrix_size(); 
    fill_matrix(&mut matrix, &matrix_size);

    println!("the input matrix:");
    matrix_print(&matrix, &matrix_size);
    
    return match parse_op(&oper) {
        Ok(op) => {
            match op {
                Operations::Add => {
                    let mut matrix2: Vec<f32> = Vec::new();
                    println!("For the second matrix:");
                    fill_matrix(&mut matrix2, &matrix_size);
                    result = matrix_add(&mut matrix, &mut matrix2);
                },
                Operations::Multiply => {
                    let mut matrix2: Vec<f32> = Vec::new();
                    println!("For the second matrix:");
                    let mut matrix2_size = set_matrix_size();
                    loop {
                        if matrix_size[1] == matrix2_size[0] {
                            break; //OK
                        }
                        else {
                            eprintln!("The input dimensions cannot be multiplied with \
                                        the dimensions of the first matrix (the first value \
                                        of the second matrix must match the second value of \
                                        the first matrix).
                                        Try again with a valid input.");
                            matrix2_size = set_matrix_size();
                        }
                    }
                    fill_matrix(&mut matrix2, &mut matrix2_size);
                    //TODO: implement updated function.
                    //result = matrix_multiply(&matrix, &matrix_size, 
                    //                         &matrix2, &matrix2_size);
                },
                Operations::ScalarAdd => {
                    let scalar = setup_scalar();
                    result = matrix_scalar_add(&mut matrix, scalar);
                },
                Operations::ScalarMultiply => {
                    let scalar = setup_scalar();
                    result = matrix_scalar_multiply(&mut matrix, scalar);
                },
                Operations::Determinant => {
                    //must ensure matrix is square
                    if matrix_size[0] != matrix_size[1] {
                        return Err(format!("determinant undefined for non-square matrices."));
                    }
                    else if matrix_size[0] == 1 {
                        return Err(format!("ERROR: \
                                    determinant undefined for matrices of size 1."));
                    }
                    result = matrix_determinant(&matrix, &mut matrix_size);
                },
                Operations::Transpose => {
                    result = matrix_transpose(&matrix, &mut matrix_size);
                    //note: mutable reference because transpose changes the dimensions
                },
                Operations::Inverse => {
                    //TODO: update implementation
                    //result = matrix_inverse(&matrix, &mut matrix_size);
                    //note: mutable reference because function calls matrix_transpose
                },
                Operations::Adjugate => {
                    //TODO: update implementation
                    //result = matrix_adjugate(&matrix, &mut matrix_size);
                    //note: mutable reference because function calls matrix_transpose
                },
            }
            matrix_print(&result, &matrix_size);
            return Ok(result)
        },
        Err(err) => Err(err),
     }
}

