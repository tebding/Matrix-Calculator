//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.
//only works for 3x3 matrices!

use std::io{self, Write}

fn main() {
    println!("Matrix calculator to operate on 3x3 matrices");
    
    loop { //calculator passively runs here
        let result = matrix::evaluate();
        match result {
            Ok(res) => {
                //matrix_print(res);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    
    
}
