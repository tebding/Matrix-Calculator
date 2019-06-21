//lib.rs
//library functions for matrix math calculator

/*
    TO UPGRADE:
    convert MATRIX_SIZE to be based on user input
        (they'd enter x and y length, x*y=M_S)
    generalize calculation functions. this is the hard part.
*/


const MATRIX_SIZE: u32 = 9;

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
   1) tokenizes the input by whitespace.
   2) each token is checked to see if it's an <f32>
   3) tokens are collected into the Result Vec<>
*/
fn parse_matrix(mat: &str) -> Result<Vec<f32>, String> {
    mat.split_whitespace().map(|num| {
        match num.parse::<f32>() {
            Ok(val) => Ok(val),
            Err(_) => Err(format!("Cannot parse value \"{}\", num")),
        }
    })
        .into_iter().collect()
}

/*
   prompts the user to enter a list of numbers, and uses them to create
   the matrix in row-major order (represented by a Vec<>)
*/
fn fill_matrix(mat: &mut Vec<f32>) {
    let mut matrixString = String::new();
    let mut err_flag = bool;
    loop {
        //reset error flag
        err_flag = false;

        //prompt user for input
        print!("> ");
        io::stdout().flush().expect("failed to flush");

        //take user input into a string
        io::stdin().read_line(&mut matrixString)
            .expect("failed to read line");
        
        //add input into Vec<> for the matrices
        match parse_matrix(matrixString) {
            Ok(matrixOk) => {
                for val in matrixOk {
                    mat.push(val);
                }
                //if there were the wrong number of values, error
                if mat.len() != MATRIX_SIZE { 
                    eprintln!("Invalid number of values");
                    println!("Enter the values for the matrix in
                             row-major order (separated by spaces):");
                    err_flag = true;
                } 
            },
            Err(err) => { //occurs when non <f32> values input
                eprintln!("One or more values have invalid input types");
                println!("Enter the values for the matrix in
                         row-major order (separated by spaces):");
                err_flag = true;
            },
        };
        //once the matrix is filled, exit the loop
        if err_flag == false {
            break;
        }
        
    }
}
 
fn setup_binary_op(m1: &mut Vec<f32>, m2: &mut Vec<f32) {
    println!("Enter the values for the first matrix in
            row-major order (separated by spaces):");
    fill_matrix(&mut m1);
    
    println!("Enter the values for the second matrix in
            row-major order (separated by spaces):");
    fill_matrix(&mut m2);
}

fn setup_scalar_op(mat: &mut Vec<f32>, scalarStr: &str) -> f32 {
    println!("Enter the values for the matrix in
            row-major order (separated by spaces):");
    fill_matrix(&mut matrix);
    //get the scalar value
    loop { //to allow re-tries if user error occurs
        println!("Enter a scalar value to apply to the matrix:");
        println!("> ");
        io::stdout().flush().expect("failed to flush");
        
        io::stdin().readline(&mut scalarStr)
            .expect("failed to read line");
        match scalarStr.parse::<f32>() { //does this *convert* scalar?
            Ok(val) => {
                Ok(val);
                break;
            },
            Err(_) => eprintln!("Invalid scalar value"),
        }
    } 
}

fn setup_unary_op(mat:&mut matrix) {
    println!("Enter the values for the matrix in
            row-major order (separated by spaces):");
    fill_matrix(&mut mat);
}

/*
   called in main.
   executes the input, setup, and calculations.
   returns the resultant matrix to main.
*/
pub fn evaluate() -> Result<Vec<f32>, String> {
    println!("Enter the operation to be performed. Valid operations include:");
    println!("Add, ScalarAdd, Multiply, ScalarMultiply, Determinate,
             Trace, Transpose, Inverse, Adjugate");
    print!("> ");
    io::stdout().flush().expect("failed to flush");
    
    let mut oper = String::new();
    io::stdin().read_line(&mut oper)
        .expect("failed to read line");
    
    //declare matrix/matrices.
    //some wasted declarations are assured, but this modularizes the functions more
    let mut matrix: Vec<f32> = Vec::new();
    let mut matrix2: Vec<f32> = Vec::new();
    let mut scalarStr = String::new();
    
    return match parse_op(oper) {
        Ok(op) {
            //run setup
            if op == (Operations::Add || Operations::Multiply) {
                setup_binary_op(&mut matrix, &mut matrix2);
                            }
            else if op == (Operations::ScalarAdd || Operations::ScalarMultiply) {
                let scalarStr = setup_scalar_op(&mut matrix, &mut scalarStr);
            }
            else { //op == Determinate, Trace, Transpose, or Adjugate
                setup_unary_op(&mut matrix);
            }
            
            let mut res =  Vec<f32>::with_capacity(MATRIX_SIZE);
            match op {
                Operations::Add => matrix_add(&matrix, &matrix2, &mut res),
                Operations::Multiply => matrix_multiply(&matrix, &matrix2, &mut res),
                Operations::ScalarAdd => matrix_scalar_add(&matrix, scalar, &mut res),
                Operations::ScalarMultiply => matrix_scalar_multiply(&matrix, scalar, &mut res),
                Operations::Determinate => matrix_determinate(&matrix),
                Operations::Trace => matrix_trace(&matrix),
                Operations::Transpose => matrix_transpose(&matrix),
                Operations::Inverse => matrix_inverse(&matrix),
                Operations::Adjugate => matrix_adjugate(&matrix),
                _ => eprintln!("invalid operation somehow made it to fn selection"),
        },
        Err(err) => Err(err),
    }

}

/*
   adds the values of each matrix's equivalent indices together
*/
fn matrix_add(m1: &Vec<f32>, m2: &Vec<f32>, res: &mut Vec<f32>){
    for i in 0..MATRIX_SIZE {
        res[i] = m1[i] + m2[i];
    }
}

/*
   performs matrix multiplication
*/
fn matrix_multiply(m1: &Vec<f32>, m2: &Vec<f32>, res: &mut Vec<f32>) {
    
}

/*
   adds a scalar value to each index of the matrix
*/
fn matrix_scalar_add(matrix: &Vec<f32>, sv: f32, res: &mut Vec<f32>) {
    f for i in 0..MATRIX_SIZE {
        res[i] = matrix[i] + sv;
    }
}

/*
   multiplies each index of the matrix by a scalar value
*/
fn matrix_scalar_multiply(matrix: &Vec<f32>, sv: f32, res: &mut Vec<f32>) {
    for i in 0..MATRIX_SIZE {
        res[i] = matrix[i] * sv;
    }
}

/*
   calculates the Determinate of the given matrix
*/
fn matrix_determinate(matrix: &Vec<f32>, res: &mut Vec<f32>){
    
}

/*
   calculates the Trace of the given matrix
*/
fn matrix_trace(matrix: &Vec<f32>, res: &mut Vec<f32>) {
    
}

/*
   calculates the Transpose of the given matrix
*/
fn matrix_transpose(matrix: &Vec<f32>, res: &mut Vec<f32>) {
    
}

/*
   calculates the Inverse of the given matrix
*/
fn matrix_inverse(matrix: &Vec<f32>, res: &mut Vec<f32>) {
    
}

/*
   calculates the Adjugate of the given matrix
*/
fn matrix_adjugate(matrix: &Vec<f32>, res: &mut Vec<f32>) {
    
}


pub fn matrix_print(matrix: Vec<f32>) {
   //probably just print 3, then \n etc 
}

