#[macro_export]
macro_rules! html_list {
    () => (
        String::new()
    );

    ($elem:expr; $n:expr) => (
        {
            let s = format!("<li>{}</li>", $elem);
            let array = vec![s; $n];
            let res = array.join("");
            format!("<ul>{}</ul>", res)
        }
    );

    ($($x:expr),+ $(,)?) => (
        {
            let mut array = vec![];
            $(
                let s = format!("<li>{}</li>", $x);
                array.push(s);
            )*
            let res = array.join("");
            format!("<ul>{}</ul>", res)
        }
    );
}

#[cfg(test)]
mod html_list_test {
    #[test]
    fn it_works() {
        let a = html_list![];
        assert_eq!(a, String::from(""));
        let aa = html_list![1; 2];
        assert_eq!(aa, String::from("<ul><li>1</li><li>1</li></ul>"));
        let aa = html_list![1, 2];
        assert_eq!(aa, String::from("<ul><li>1</li><li>2</li></ul>"));
        let aa = html_list!["1", 2];
        assert_eq!(aa, String::from("<ul><li>1</li><li>2</li></ul>"));
        let aa = html_list!["1"; 2];
        assert_eq!(aa, String::from("<ul><li>1</li><li>1</li></ul>"));
    }
}