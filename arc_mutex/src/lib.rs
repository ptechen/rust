#[cfg(test)]
mod tests {
    use super::arc_mutex_example;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        arc_mutex_example();
    }
}

use std::sync::{Arc, Mutex};
use std::thread;

fn arc_mutex_example() {
    let vec = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0..5 {
        let v = vec.clone();
        let t = thread::spawn(move||{
            let mut v = v.lock().unwrap();
            v.push(i)
        });
        childs.push(t);
    }
    for c in childs {
        c.join().unwrap();
    }
    println!("{:?}", vec)
}