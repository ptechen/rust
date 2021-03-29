use hello_macro_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro() ->String;
}

#[derive(HelloMacro)]
#[HelloWorldName = "the best Hello1"]
struct Hello;

fn main() {
    let s = Hello::hello_macro();
    println!("{}", s)
}