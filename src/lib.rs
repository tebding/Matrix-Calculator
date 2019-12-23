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
    Transpose,
    Inverse,
    Adjugate,
    Exit,
}


/*******************************************
           OPERATIONAL FUNCTIONS
*******************************************/

//translates input string into an Operations enum
pub fn parse_op(expr: &str) -> Result<Operations, String> {
    match expr.to_ascii_lowercase().trim() {
        "add" => Ok(Operations::Add),
        "scalar add" => Ok(Operations::ScalarAdd),
        "multiply" | "mult" => Ok (Operations::Multiply),
        "scalar multiply" | "scalar mult" => Ok(Operations::ScalarMultiply),
        "determinant" | "det" => Ok(Operations::Determinant),
        "transpose" | "trans" => Ok(Operations::Transpose),
        "inverse" | "inv" => Ok(Operations::Inverse),
        "adjugate" | "adj" => Ok(Operations::Adjugate),
        "exit" => Ok(Operations::Exit), //instead of an operation, exits the main loop
        _ => Err(format!("Cannot parse operation")),
    }
}


/*
   1) tokenizes the input (by whitespaces)
   2) each token is checked to see if it's an f32
   3) tokens are colected into the vec Result
*/
fn parse_matrix(matrix: &str) -> Result<Vec<f32>, String> {
    matrix.split_whitespace().map(|num| {
        match num.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("value \"{}\" is not a number", num))
        }
    })
        .into_iter().collect()
}


//as parse_matrix, but for matrix size.
//while a generic to cover both functions would be nicer, it's not worth implementing...
fn parse_size(size: &str) -> Result<Vec<usize>, String> {
    size.split_whitespace().map(|num| {
        match num.parse::<usize>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("value \"{}\" is not valid", num)),
        }
    })
        .into_iter().collect()
}

/*
   prompts the user to enter a list of numbers, and uses them to create
   the matrix in row-major order (represented by a vec)
*/
pub fn fill_matrix(matrix: &mut Vec<f32>, size: &Vec<usize>) {
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
                if matrix_ok.len() != (size[2]) {
                    eprintln!("Invalid number of values");
                }
                else {
                    while matrix_ok.is_empty() == false {
                        matrix.push(matrix_ok.remove(0));
                    }
                    //the matrix is now filled.
                    break;
                }
            },
            Err(err) => eprintln!("{}", err), //occurs when input vals are not f32s
        }
    }
}

//asks the user for input, then parses and sets matrix size from it
pub fn set_matrix_size() -> Vec<usize> {
    loop { //to allow for re-tries in case of user error
        // I/O
        print!("enter size\n> ");
        io::stdout().flush().unwrap();
        let mut size_str = String::new();
        io::stdin().read_line(&mut size_str).expect("error reading line");
        
        match parse_size(&size_str) {
            Ok(mut size_ok) => {
                if size_ok.len() != 2 {
                    eprintln!("must have exactly 2 dimensions");
                }
                else {
                    size_ok.push(size_ok[0] * size_ok[1]); //set 3rd index to total capacity
                    return size_ok;
                }
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}

//takes user-inputted value to be applied as a scalar
pub fn setup_scalar() -> f32 {  
    let mut scalar_str = String::new();
    //get the scalar value
    loop { //to allow re-tries in case of errors
        print!("Enter a scalar value to apply to the matrix:\n> ");
        io::stdout().flush().expect("failed to flush");
        
        io::stdin().read_line(&mut scalar_str)
            .expect("failed to read line");
        match scalar_str.trim().parse::<f32>() {
            Ok(val) => {
                return val; 
            },
            Err(_) => {
                eprintln!("invalid scalar value");
                scalar_str.clear();
            }
        }
    }
}


//calculates and returns the matrix minor for the given matrix and column
fn get_matrix_minor(matrix: &Vec<f32>, size: &Vec<usize>, column: &usize) -> Vec<f32> {
    let rows = size[0];
    let cols = rows;
    let mut res: Vec<f32> = Vec::new();
    
    //k is an offset to have first elements added be from the 2nd row
    let mut k: usize = cols;
    for _i in 1..rows {
        for j in 0..cols {
            if j != *column {
                res.push(matrix[(j as usize)+k]);
            }
        }
        k += rows;
    }
    res
}

//prints a matrix in 2D format (hardcoded)
pub fn matrix_print(matrix: &Vec<f32>, size: &Vec<usize>) {
    let mut i: usize = 0;
    let rows = size[0];
    let cols = size[1];
    for _j in 0..rows {
        print!("[ "); //each line begins with an opening bracket
        for _k in 0..cols {
            print!("{} ", matrix[i]);
            i += 1;
        }
        println!("]"); //each line ends with a closing bracket
    }
    println!(""); //adds extra blank line after matrix for better visual spacing
}


/************************************
        CALCULATION FUNCTIONS
************************************/

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
    
    //by transposing the second matrix the abstraction requires fewer translations
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
    //the output will be a vector of length 1.
pub fn matrix_determinant(matrix: &Vec<f32>, size: &Vec<usize>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(1);
    if size[0] == 2 {
        let det = (matrix[0] * matrix[3]) - (matrix[1] * matrix[2]);
        res.push(det);
        res
    }
    else if size[0] > 2 {
        let cols = size[1];
        let mut det: f32 = 0.0;
        for i in 0..cols {
            let matrix_minor = get_matrix_minor(&matrix, &size, &i);
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
    else { //????
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


/*
   calculates the Inverse of the given matrix
   1) calculates the inverse of the determinant
   2) multiples it by the adjugate of the matrix
      which yields the inversed matrix
*/
pub fn matrix_inverse(matrix: &Vec<f32>, mut size: &mut Vec<usize>) -> Vec<f32> {
    let det = matrix_determinant(&matrix, &size);
    let inv_det = 1.0 / (det[0] as f32);
    let mut adj = matrix_adjugate(&matrix, &mut size);
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
pub fn matrix_adjugate(matrix: &Vec<f32>, mut size: &mut Vec<usize>) -> Vec<f32> {
    let mut res: Vec<f32> = Vec::with_capacity(size[2]);
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
    res = matrix_transpose(&res, &mut size);
    res
}

