//lib.rs 
//contains library functions for matrix math calculator

use std::io::{self, Write};
const MATRIX_SIZE: u32 = 9;

//to help use the operation type as a variable
enum Operations {
    Add,
    ScalarAdd,
    Multiply,
    ScalarMultiply,
    Determinate,
    Trace,
    Transpose,
    Inverse,
    Adjugate,
}


//translates input string into an Operations enum
fn parse_op(expr: &str) -> Result<Operations, String> {
    match expr.trim() {
        "Add" => Ok(Operations::Add),
        "ScalarAdd" => Ok(Operations::ScalarAdd),
        "Multiply" => Ok (Operations::Multiply),
        "ScalarMultiply" => Ok(Operations::ScalarMultiply),
        "Determinate" => Ok(Operations::Determinate),
        "Trace" => Ok(Operations::Trace),
        "Transpose" => Ok(Operations::Transpose),
        "Inverse" => Ok(Operations::Inverse),
        "Adjugate" => Ok(Operations::Adjugate),
        _ => Err(format!("Cannot parse operation")),
    }
}


/*
   1) tokenizes the input (by whitespaces)
   2) each token is checked to see if it's an f32
   3) tokens are colected into the vec Result
*/
/*
fn parse_matrix(mat: &str) -> Result<Vec<f32>, String> {
    //mat.split_whitespace().map(|num| {}
}
*/

/*
   prompts the user to enter a list of numbers, and uses them to create
   the matrix in row-major order (represented by a vec)
*/
//fn fill_matrix(mat: &mut Vec<f32>) {
    
//}


fn setup_binary_op(m1: &mut Vec<f32>, m2: &mut Vec<f32>) {
    println!("Enter the values for the first matrix in row-major order (separated by spaces):");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    //fill_matrix(&mut m1)
    //TEST CODE
    let mut foo = String::new();
    io::stdin().read_line(&mut foo).expect("failed to read line");
    println!("finished reading 1st matrix. foo = {}", foo);
    
    println!("Enter the values for the second matrix in row-major order (separated by spaces):");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    //fill_matrix(&mut m2)
    //TEST CODE
    let mut bar = String::new();
    io::stdin().read_line(&mut bar).expect("failed to read line");
    println!("finished reading 2nd matrix. bar = {}", bar);
}


fn setup_scalar_op(mat: &mut Vec<f32>) -> f32 {  
    println!("Enter the values for the matrix in row-major order (separated by spaces):");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    //fill_matrix(&mut mat)
    
    let mut scalarStr = String::new();
    //get the scalar value
    loop { //to allow re-tries in case of errors
        println!("Enter a scalar value to apply to the matrix:");
        print!("> ");
        io::stdout().flush().expect("failed to flush");
        
        io::stdin().read_line(&mut scalarStr)
            .expect("failed to read line");
        match scalarStr.trim().parse::<f32>() {
            Ok(val) => {
                return val; //does this return immediately, or do I need to call break?
            },
            Err(_) => eprintln!("invalid scalar value"),
        }
    }
}


fn setup_unary_op(mat: &mut Vec<f32>) {
    println!("Enter the values for the matrix in
            row-major order (separated by spaces):");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    //fill_matrix(&mut mat);
    //TEST CODE
    let mut foobar = String::new();
    io::stdin().read_line(&mut foobar).expect("failed to read line");
    println!("finished reading unary matrix. foobar = '{}'", foobar);
}


//prints a matrix in 2D format
pub fn matrix_print(matrix: &Vec<f32>) {
}


/*
   called in main().
   executes the input, setup, and calculations.
   returns the resultant matrix to main().
*/
pub fn evaluate() -> Result<Vec<f32>, String> {
    println!("\nEnter the operation to be performed (CTRL+C to exit). Valid operations include:");
    println!("Add, ScalarAdd, Multiply, ScalarMultiply, Determinate, Transpose, Inverse, Adjugate");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    
    let mut oper = String::new();
    io::stdin().read_line(&mut oper)
    .expect("failed to read line");
    
    //the matrix to operate on. For scalar/binary ops, other vars assigned later
    let mut matrix: Vec<f32> = Vec::new();
    let mut result: Vec<f32> = Vec::new();

     return match parse_op(&oper) {
        Ok(op) => {
            println!("\nOperaton parsed: {}", &oper);
            match op {
                Operations::Add | Operations::Multiply => {
                    let mut matrix2: Vec<f32> = Vec::new();
                    setup_binary_op(&mut matrix, &mut matrix2);
                    /*if op == Operations::Add {
                        result = matrix_add(&matrix, &matrix2);
                    }
                    else {
                        result = matrix_multiply(&matrix, &matrix2
                    }*/
                },
                Operations::ScalarAdd | Operations::ScalarMultiply => {
                    let scalar = setup_scalar_op(&mut matrix);
                    println!("exited setup_scalar. scalar = {}", scalar);
                    /*if op == Operations::ScalarAdd {
                        result = matrix_scalar_add(&matrix, &scalar);
                    }
                    else {
                        result = matrix_scalar_multiply(&matrix, &scalar);
                    }*/
                },
                _ => {
                    setup_unary_op(&mut matrix);
                    /*match op {
                        //Operations::Determinate => {result = matrix_determinate(&matrix);},
                        Operations::Transpose => {
                            result = matrix_transpose(&matrix);
                        },
                        Operations::Inverse => {
                            result = matrix_transpose(&matrix);
                        },
                        Operations::Adjugate => {
                            result = matrix_adjugate(&matrix);
                        },
                    }*/
                }
            }
            
            let test_mat: Vec<f32> = vec![1.0, 2.5, 4.2];
            return Ok(test_mat)
        },
        Err(err) => Err(err),
     }
}


//MATRIX CALCULATION FUNCTIONS

//adds the values of each matrix's equivalent indices together
/*fn matrix_add() {}


//performs matrix multiplication
fn matrix_multiply() {}


//adds the given value to each index in the matrix
fn matrix_scalar_add() {}


//multiplies each index of the matrix by a scalar value
fn matrix_scalar_multiply() {}


//calculates the Determinate of the given matrix
fn matrix_determinate() {}


//calculates the Transpose of the given matrix
fn matrix_transpose() {}


//helper function?


/*
   calculates the Inverse of the given matrix
   1) calculates the inverse of the determinate
   2) multiples it by the adjugate of the matrix
      which yields the inversed matrix
*/
fn matrix_inverse() {}


/*
   calculates the Adjugate of the given matrix
   1) calculates the cofactor matrix by finding the
      matrix minors for each index of the input matrix.
   2) transposes he resultant cofactor matrix, which
      yields the adjugate matrix that we proceed to return
*/
*/
