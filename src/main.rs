//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.
//only works for 3x3 matrices!

use Matrix_Calculator::{evaluate, matrix_print};

fn main() {
    println!("Matrix calculator to operate on 3x3 matrices");
    
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
