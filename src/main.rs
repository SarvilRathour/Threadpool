use std::fmt::{Display,Debug};
trait DisplayDebug: Display + Debug{}
impl<T:Display+Debug> DisplayDebug for T{}
fn foo(a:&(dyn DisplayDebug)){
    println!("{:?}",a);
    println!("hello");
}
fn test_foo(){
    foo(&1u8);
    foo(&String::new());
}
fn main() {
    test_foo();
    println!("Hello, world!");
}
