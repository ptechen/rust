#[macro_export]
macro_rules! scanline {
    ($x:expr) => {
        std::io::stdin().read_line(&mut $x).unwrap()
    };

    () => {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::scanline;
    #[test]
    fn it_works() {
        let mut input = String::new();
        scanline!(input);
        assert_eq!("\n".to_string(), input);
    }
}

#[cfg(test)]
mod scanline_test {
    use crate::scanline;
    #[test]
    fn it_works() {
        scanline!();
    }
}