# Matrix-Calculator
Rust implementation of a matrix calculator program.

My goal was to improve upon an old C project I did that did any of the functions seen here, but required the input matrices to be written into the source code and was strictly limited to working with 3x3 matrices.

Taking in user input proved to be the more challenging and interesting aspect of this project. I did my best to keep each individual part as modular as possible (without fracturing the functionality to the point that reading it is a challenge).

Design decisions and appraoch are mentioned in comments throughout the code to explain how it works at a granular level.

The overhaul to operate on matrices of any size was also a fun challenge, albeit less complex than originally estimated.
Regardless, the `variable-size-matrices` branch has been merged to `master`.

FUTURE WORK:
I'm considering building a web app for this to run with, which could have an even more user-friendly UX.
I'm also thinking about adding unit tests to practice utilizing the feature provided in Rust.
