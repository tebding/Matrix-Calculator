# Matrix-Calculator
Rust implementation of a matrix calculator program.

My goal was to improve upon an old C project I did that did any of the functions seen here, but required the input matrices to be written into the source code and is strictly limited to working with 3x3 matrices.

Taking in user input proved to be the more challenging and interesting aspect of this project. I did my best to keep each individual part as modular as possible without fracturing the functionality to the point that reading it is a challenge.

The biggest style improvement I could think of would perhaps be to streamline the evaluate() function, though that would potentially just fragment the code unnecessarily.

Design decisions and appraoch are mentioned briefly in comments throughout the code to explain how it works at a granular level.

POSSIBLE future work would include upgrading it to operate on any (valid) input matrix.
The design decisions for IO and methodology would be best if based on (hypothetical) users' experience. However the main reason I didn't initially commit to this task was due to the steep increase in complexity to design an algorithm that can operate on any size (even any size square matrix would require some considerable thought).
