#[cfg(test)]
mod tests {
    use super::arc_example;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        arc_example()
    }
}
use std::thread;
use std::sync::Arc;
fn arc_example() {
    let nums = Arc::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let mut childs = vec![];
    for n in 0..5 {
        let ns = Arc::clone(&nums);
        let c = thread::spawn(move||{
            println!("{}", ns[n]);
        });
        childs.push(c);
    }
    for c in childs {
        c.join().unwrap();
    }
}