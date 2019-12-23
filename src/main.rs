//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.
//only works for 3x3 matrices!

use matrix_calculator::{*};
use std::io::{self, Write};

fn main() {
    println!("Matrix calculator program initializing...");
     
    loop { //calculator passively runs here
        print!("Enter the operation to be performed (CTRL+C to exit). Valid operations include:\n\
                Add, Scalar Add, Multiply, Scalar Multiply, Determinant, Transpose, Inverse, Adjugate\n\
                > ");
        io::stdout().flush().expect("failed to flush");
        
        let mut oper = String::new();
        io::stdin().read_line(&mut oper)
        .expect("failed to read line");
        
        let mut matrix: Vec<f32> = Vec::new();
        let mut result: Vec<f32> = Vec::new();
        let mut matrix_size: Vec<usize> = Vec::with_capacity(3);
        
        match parse_op(&oper) {
            Ok(op) => {
                match op {
                    Operations::Exit => {
                    //note: this is before anything else to prevent wasting time with inputs
                        println!("exiting program.");
                        break;
                    },
                    Operations::Determinant => {
                    //note: this is in separate logic in order to to check for valid size without
                    //wasting the user's time inputting values into the invalid matrix
                        loop {
                            matrix_size = set_matrix_size();
                            if matrix_size[2] == 1 { //matrix size is 1
                                eprintln!("error: determinant undefined for matrices of size 1");
                            }
                            else if matrix_size[0] != matrix_size[1] { //non-square matrix
                                eprintln!("error: determinant undefined for non-square matrices");
                            }
                            else { //conditions met
                                break; //OK
                            }
                        }
                        fill_matrix(&mut matrix, &matrix_size);
                        println!("the input matrix:");
                        matrix_print(&matrix, &matrix_size);
                        result = matrix_determinant(&matrix, &matrix_size);
                        matrix_size[0] = 1;
                        matrix_size[1] = 1;
                        matrix_size[2] = 1; //manually setting dimensions/len of result to 1
                    },
                    _ => { //all the binary and other unary ops
                        matrix_size = set_matrix_size();
                        fill_matrix(&mut matrix, &matrix_size);
                        println!("the input matrix:");
                        matrix_print(&matrix, &matrix_size);
                        match op {
                            Operations::Add => {
                                let mut matrix2: Vec<f32> = Vec::new();
                                println!("For the second matrix:");
                                fill_matrix(&mut matrix2, &matrix_size);
                                println!("the input 2nd matrix:");
                                matrix_print(&matrix2, &matrix_size);
                                
                                result = matrix_add(&mut matrix, &mut matrix2);
                            },
                            Operations::Multiply => {
                                let mut matrix2: Vec<f32> = Vec::new();
                                let mut matrix2_size: Vec<usize> = Vec::with_capacity(3);
                                println!("For the second matrix:");
                                loop {
                                    matrix2_size = set_matrix_size();
                                    if matrix_size[1] != matrix2_size[0] {
                                        eprintln!("The input dimensions cannot be multiplied with \
                                                    the dimensions of the first matrix (the first \
                                                    value of the second matrix must match the \
                                                    second value of the first matrix).
                                                    Try again with a valid input.");
                                    }
                                    else {
                                        break; //OK
                                    }
                                }
                                fill_matrix(&mut matrix2, &matrix2_size);
                                println!("the input 2nd matrix:");
                                matrix_print(&matrix2, &matrix2_size);
                                
                                //TODO result = matrix_multiply(&matrix, &mut matrix_size,
                                //                                &matrix2, &mut matrix2_size);
                            },
                            Operations::ScalarAdd => {
                                let scalar = setup_scalar();
                                result = matrix_scalar_add(&mut matrix, scalar);
                            },
                            Operations::ScalarMultiply => {
                                let scalar = setup_scalar();
                                result = matrix_scalar_multiply(&mut matrix, scalar);
                            },
                            Operations::Transpose => {
                                result = matrix_transpose(&matrix, &mut matrix_size);
                            },
                            Operations::Inverse => {
                                //TODO result = matrix_inverse(&matrix, &mut matrix_size);
                            },
                            Operations::Adjugate => {
                                //TODO result = matrix_adjugate(&matrix, &mut matrix_size);
                            },
                            _ => { //???
                                panic!("this should be uncreachable???");
                            },
                        } //end inner match op
                    },
                } //end outer match op
                println!("Result:");
                matrix_print(&result, &matrix_size);
            },
            Err(err) => {
                eprintln!("error: {}", err);
            },
        } //end match parse_op()
    } //end loop
} //end main
