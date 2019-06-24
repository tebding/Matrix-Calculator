//lib.rs 
//contains library functions for matrix math calculator


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
fn parse_matrix(mat: &str) -> Result<Vec<f32>, String> {
    //mat.split_whitespace().map(|num| {}
}

/*
   prompts the user to enter a list of numbers, and uses them to create
   the matrix in row-major order (represented by a vec)
*/
fn fill_matrix(mat: &mut Vec<f32>) {
    
}


fn setup_binary_op(m1: &mut Vec<f32>, m2: &mut Vec<f32>) {

}


fn setup_scalar_op(mat: &mut Vec<f32>, scalarStr: &str) -> f32 {
    
}


fn setup_unary_op(mat: &mut matrix) {
    println!("Enter the values for the matrix in
            row-major order (separated by spaces):");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    fill_matrix(&mut mat);

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
    println!("Enter the operation to be performed. Valid operations include:");
    println!("Add, ScalarAdd, Multiply, ScalarMultiply, Determinate,
             Transpose, Inverse, Adjugate");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    
    let mut oper = String::new();
    io::stdin().read_line(&mut oper)
    .expect("failed to read line");
    
     return match parse_op(oper) {
        Ok(op) {
            //run setup
        },
        Err(err) => Err(err),
     }
}


//MATRIX CALCULATION FUNCTIONS

//adds the values of each matrix's equivalent indices together
fn matrix_add() {}


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
