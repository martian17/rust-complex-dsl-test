use dsl_parser::{test_constant, complex};

#[cfg(test)]
mod tests {
    use super::*;
    use num::complex::Complex;

    #[test]
    fn it_works() {
        let result = test_constant!(2.4*5+3i);
        assert_eq!(result, Complex::new(2.3,3.4));
    }
    #[test]
    fn it_works2() {
        let result = complex!(2.4*5+3i);
        //let result = complex!(2.2);
        assert_eq!(result, Complex::new(12.0,3.0));
    }
}

fn main() {
    println!("Hello, world!");
}
