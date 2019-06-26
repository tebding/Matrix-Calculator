//main.rs
//matrix math calculator
//takes an input matrix or matrices and operation to run on the matrix/matrices.
//only works for 3x3 matrices!

use Matrix_Calculator::evaluate;

fn main() {
    println!("Matrix calculator to operate on 3x3 matrices");
    
    loop { //calculator passively runs here
        let res = evaluate();
        match res {
            Ok(res) => {
                //matrix_print(res);
                println!("Test: in main: match res: Ok =>. test_mat: {}, {}, {}", res[0], res[1], res[2]);
            },
            Err(err) => eprintln!("Error: {}", err),
        }
    }
    
    
}
