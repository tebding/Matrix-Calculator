//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.
//only works for 3x3 matrices!

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
                    matrix_print(&res);
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
            Add, ScalarAdd, Multiply, ScalarMultiply, Determinant, Transpose, Inverse, Adjugate\n\
            > ");
    io::stdout().flush().expect("failed to flush");
    
    let mut oper = String::new();
    io::stdin().read_line(&mut oper)
    .expect("failed to read line");
    
    //the matrix to operate on. For scalar/binary ops, other vars assigned later
    let mut matrix: Vec<f32> = Vec::new();
    let mut result: Vec<f32> = Vec::new();

     return match parse_op(&oper) {
        Ok(op) => {
            match op {
                Operations::Add => {
                    let mut matrix2: Vec<f32> = Vec::new();
                    println!("For the first matrix:");
                    fill_matrix(&mut matrix);
                    println!("For the second matrix:");
                    fill_matrix(&mut matrix2);
                    result = matrix_add(&mut matrix, &mut matrix2);
                },
                Operations::Multiply => {
                    let mut matrix2: Vec<f32> = Vec::new();
                    println!("For the first matrix:");
                    fill_matrix(&mut matrix);
                    println!("For the second matrix:");
                    fill_matrix(&mut matrix2);
                    result = matrix_multiply(&matrix, &matrix2);
                },
                Operations::ScalarAdd => {
                    fill_matrix(&mut matrix);
                    let scalar = setup_scalar();
                    result = matrix_scalar_add(&mut matrix, scalar);
                },
                Operations::ScalarMultiply => {
                    fill_matrix(&mut matrix);
                    let scalar = setup_scalar();
                    result = matrix_scalar_multiply(&mut matrix, scalar);
                },
                _ => {
                    fill_matrix(&mut matrix);
                    match op {
                        Operations::Determinant => {
                            result = matrix_determinant(&matrix);
                        },
                        Operations::Transpose => {
                            result = matrix_transpose(&matrix);
                        },
                        Operations::Inverse => {
                            result = matrix_inverse(&matrix);
                        },
                        Operations::Adjugate => {
                            result = matrix_adjugate(&matrix);
                        },
                        _ => eprintln!("how did this happen"),
                    }
                }
            }
            return Ok(result)
        },
        Err(err) => Err(err),
     }
}

