//lib.rs
//contains library functions for matrix math calculator

use std::io::{self, Write};

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
    Blank,
}


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
        "adjugate" | "adjoint" | "adj" => Ok(Operations::Adjugate),
        "exit" => Ok(Operations::Exit), //instead of an operation, exits the main loop
        "" => Ok(Operations::Blank), //if the user presses 'enter' with blank input
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
//COULD make a function to take in generics, but would require lots of work to save no time at all
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


//calculates and returns the matrix minor for the given matrix and row/column
fn get_matrix_minor(matrix: &Vec<f32>, size: &Vec<usize>, row_col: Vec<usize>) -> Vec<f32> {
    let rows = size[0];
    let cols = rows;
    let mut res: Vec<f32> = Vec::new();
    
    let mut k: usize = 0;
    for i in 0..rows {
        if i != row_col[0] {
            for j in 0..cols {
                if j != row_col[1] {
                    res.push(matrix[k]);
                }
                k += 1;
            }
        }
        else {
            k += cols; //increment k by the number of columns skipped
        }
    }
    res
}

//prints a matrix in 2D format
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_op_test() {
        
    }
    
    #[test]
    fn parse_matrix_test() {
        
    }
    
    #[test]
    fn parse_size_test() {
        
    }
    
    #[test]
    fn fill_matrix_test() {
        
    }
    
    #[test]
    fn set_matrix_size_test() {
        
    }
    
    #[test]
    fn setup_scalar_test() {
        
    }
    
    #[test]
    fn get_matrix_minor_test() {
        
    }
 
    #[test]
    fn matrix_print_test() {
        
    }
    
}
