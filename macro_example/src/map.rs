macro_rules! map {
    () => {
        std::collections::HashMap::new()
    };
    ($($k:expr => $v:expr),*) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($k, $v);
            )*
            map
        }
    };
}

#[cfg(test)]
mod map_test {
    #[test]
    fn it_works() {
        let a = map!(0 => 1);
        assert_eq!(a[&0], 1);
        let mut aa = map!();
        aa.insert(0, 1);
        assert_eq!(aa[&0], 1);
    }
}