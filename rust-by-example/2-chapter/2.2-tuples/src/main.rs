use std::fmt;

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "( {} {} )", self.0, self.1)?;
        writeln!(f, "( {} {} )", self.2, self.3)
    }
}

fn main() {
    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    // println!("{:?}", matrix);
    println!("Matrix:\n{}", matrix);
    println!("Transpose:\n{}", transposed(matrix));
}

fn transposed(matrix: Matrix) -> Matrix {
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}
