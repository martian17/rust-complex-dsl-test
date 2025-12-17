use dsl_parser::{poc_macro, complex_v1, complex_v2};

#[cfg(test)]
mod tests {
    use super::*;
    use num_complex::Complex;

    #[test]
    fn it_works() {
        let result = poc_macro!(2.4*5+3i);
        assert_eq!(result, Complex::new(2.3,3.4));
    }
    #[test]
    fn it_works1() {
        let result = complex_v1!(2.4*5+3i);
        //let result = complex_v1!(2.2);
        assert_eq!(result, Complex::new(12.0,3.0));
    }
    #[test]
    fn it_works2() {
        let result = complex_v2!(2.4*5+3i);
        //let result = complex_v1!(2.2);
        assert_eq!(result, Complex::new(12.0,3.0));
    }
}

fn main() {
    println!("Hello, world!");
}
