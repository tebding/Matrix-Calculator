//lib.rs 
//contains library functions for matrix math calculator

use std::io::{self, Write};
const MATRIX_SIZE: u32 = 9;

//to help use the operation type as a variable
pub enum Operations {
    Add,
    ScalarAdd,
    Multiply,
    ScalarMultiply,
    Determinant,
    Trace,
    Transpose,
    Inverse,
    Adjugate,
}


//translates input string into an Operations enum
pub fn parse_op(expr: &str) -> Result<Operations, String> {
    match expr.trim() {
        "Add" => Ok(Operations::Add),
        "ScalarAdd" => Ok(Operations::ScalarAdd),
        "Multiply" => Ok (Operations::Multiply),
        "ScalarMultiply" => Ok(Operations::ScalarMultiply),
        "Determinant" => Ok(Operations::Determinant),
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
pub fn parse_matrix(mat: &str) -> Result<Vec<f32>, String> {
    mat.split_whitespace().map(|num| {
        match num.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("value \"{}\" is not a number", num))
        }
    })
        .into_iter().collect()
}


/*
   prompts the user to enter a list of numbers, and uses them to create
   the matrix in row-major order (represented by a vec)
*/
pub fn fill_matrix(mat: &mut Vec<f32>) {
    loop {
        //prompt user for input
        print!("Enter the values for the matrix in row-major order \
               (separated by spaces):\n> ");
        io::stdout().flush().expect("failed to flush");
        
        let mut matrix_string = String::new();
        //take user input into a string
        io::stdin().read_line(&mut matrix_string)
            .expect("failed to read line");
        
        /*
           1) parse the input string into f32 values
           2) collect them into a Vec that will represent the matrix
        */
        match parse_matrix(&matrix_string) {
            Ok(mut matrix_ok) => {
               
                //if the number of values is incorrect, errors + restarts
                if matrix_ok.len() != (MATRIX_SIZE as usize) {
                    eprintln!("Invalid number of values");
                    matrix_ok.clear();
                }
                else {
                    while matrix_ok.is_empty() == false {
                        //I couldn't index this properly, so using push() and pop()
                        //was the only solution I could devise that worked.
                        mat.push(matrix_ok.pop().expect("failed to pop"));
                    }
                    //the matrix is in reverse order at this point. reverse() fixes it
                    return mat.reverse();
                }
            },
            Err(mut err) => { //occurs when input vals are not f32s
                eprintln!("{}", err);
                err.clear();
            }
        }
    }
}


pub fn setup_scalar() -> f32 {  
    let mut scalar_str = String::new();
    //get the scalar value
    loop { //to allow re-tries in case of errors
        println!("Enter a scalar value to apply to the matrix:\n> ");
        io::stdout().flush().expect("failed to flush");
        
        io::stdin().read_line(&mut scalar_str)
            .expect("failed to read line");
        match scalar_str.trim().parse::<f32>() {
            Ok(val) => {
                return val; //does this return immediately, or do I need to call break?
            },
            Err(_) => {
                eprintln!("invalid scalar value");
                scalar_str.clear();
            }
        }
    }
}


//prints a matrix in 2D format (hardcoded)
pub fn matrix_print(matrix: &Vec<f32>) {
    println!("[{} {} {}]", matrix[0], matrix[1], matrix[2]);
    println!("[{} {} {}]", matrix[3], matrix[4], matrix[5]);
    println!("[{} {} {}]\n", matrix[6], matrix[7], matrix[8]);
}


/*
    CALCULATION FUNCTIONS
*/

//adds the values of each matrix's equivalent indices together
pub fn matrix_add(mat1: &mut Vec<f32>, mat2: &mut Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    while mat1.is_empty() != true {
        res.push(mat1.pop().expect("foo") + mat2.pop().expect("bar"));
    }
    res.reverse();
    res
}


//performs matrix multiplication
pub fn matrix_multiply(mat1: &Vec<f32>, m2: &Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();

    //by transposing the second matrix the abstraction requires fewer translations
    let mat2 = matrix_transpose(&m2);
    let mut j = 0; //to track the first index of the current row
    let mut k = 0; //to track the index of the second matrix
    let mut temp: f32 = 0.0;
    
    for i in 0..MATRIX_SIZE {
        if (i%3 == 0) && (i != 0) { //when at a new row: 
            j += 3; //our row-marker is increased
            k = 0; //our 2nd matrix tracker is reset
        }
        //temp will have all 3 components added, then applied to res[i]
        
        for _l in 0..3 { //loop for 3. hard-coded for sqrt(MATRIX_SIZE)
        //row-starter + 2nd matrix column = which 1st matrix index to use
            temp += mat1[j+(k%3)] * mat2[k];
            k += 1;
        }
        res.push(temp);
        temp = 0.0;
    }
    res
}


//adds the given value to each index in the matrix
pub fn matrix_scalar_add(matrix: &mut Vec<f32>, scalar: f32) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    while matrix.is_empty() != true {
        res.push(matrix.pop().expect("bleh") + scalar);
    }
    res.reverse();
    res
}


//multiplies each index of the matrix by a scalar value
pub fn matrix_scalar_multiply(matrix: &mut Vec<f32>, scalar: f32) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    while matrix.is_empty() != true {
        res.push(matrix.pop().expect("bah") * scalar);
    }
    res.reverse();
    res
}


//calculates the Determinant of the given matrix
    //the output will be a vector of length 1.
pub fn matrix_determinant(matrix: &Vec<f32>) -> Vec<f32> {
    //hard-coded feels bad :(
    //if I ever change this to operate on aribitrary-size matrices I can fix it
    //recursively calculate determinants, alternate + and - until done.
    let a = matrix[0] * (matrix[4]*matrix[8] - matrix[5]*matrix[7]);
    let b = matrix[1] * (matrix[3]*matrix[8] - matrix[5]*matrix[6]);
    let c = matrix[2] * (matrix[3]*matrix[7] - matrix[4]*matrix[6]);
    let d = a - b + c;
    let mut res: Vec<f32> = Vec::new();
    res.push(d);
    res
}


//calculates the Transpose of the given matrix
pub fn matrix_transpose(matrix: &Vec<f32>) -> Vec<f32> {
    //hard-coding for now. perhaps recursive solution possible.
    let mut res: Vec<f32> = Vec::new();
    res.push(matrix[0]);
    res.push(matrix[3]);
    res.push(matrix[6]);
    res.push(matrix[1]);
    res.push(matrix[4]);
    res.push(matrix[7]);
    res.push(matrix[2]);
    res.push(matrix[5]);
    res.push(matrix[8]);
    res
}


/*
   calculates the Inverse of the given matrix
   1) calculates the inverse of the determinant
   2) multiples it by the adjugate of the matrix
      which yields the inversed matrix
*/
pub fn matrix_inverse(matrix: &Vec<f32>) -> Vec<f32> {
    let det = matrix_determinant(&matrix);
    let inv_det = 1.0 / (det[0] as f32);
    let mut adj = matrix_adjugate(&matrix);
    let res = matrix_scalar_multiply(&mut adj, inv_det);
    res
}


/*
   calculates the Adjugate of the given matrix
   1) calculates the cofactor matrix by finding the
      matrix minors for each index of the input matrix.
   2) transposes he resultant cofactor matrix, which
      yields the adjugate matrix that we proceed to return
*/
pub fn matrix_adjugate(matrix: &Vec<f32>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::new();
    //unfortunately hard-coded to the initial positions of
    //determinant component vars (for a 3x3 matrix)
    let mut a = 4;
    let mut b = 8;
    let mut c = 7;
    let mut d = 5;
    

    for i in 0..MATRIX_SIZE {
        if i%2 == 1 { //if i is odd
            res.push(-1.0 * ((matrix[a]*matrix[b])-(matrix[c]*matrix[d])));
        }
        else {
            res.push((matrix[a]*matrix[b])-(matrix[c]*matrix[d]));
        }
    
    /*
       this 'match' section is based on the movement of the
       determinant's component vars as the index of the matrix minor being
       calculated moves through the Vec that represents the matrix
    */
        match i {
            0 | 3 | 6 => {
                a -= 1;
                c -= 1;
                if i == 3 {
                    //res.push(res.pop().expect("whoops") * -1.0)
                }
            },
            1 | 4 | 7 => {
                b -= 1;
                d -= 1;
                if i != 4 {
                    //res.push(res.pop().expect("whoops") * -1.0);
                }
            },
            2 => {
                a -= 2;
                b += 1;
                c += 1;
                d -= 2;
            },
            5 => {
                a += 1;
                b -= 2;
                c -= 2;
                d += 1;
                //res.push(res.pop().expect("whoops") * -1.0);
            },
            _ => (),
        };
    }
    res = matrix_transpose(&res);
    res
}

