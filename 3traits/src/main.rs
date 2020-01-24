mod print_trait;

use print_trait::{MyOtherTrait, MyPrint};

fn call_print<T: MyPrint>(v: &T) {
    v.my_print()
}

#[derive(Debug)]
pub struct MyStruct;
impl MyOtherTrait for MyStruct {}

impl MyStruct {
    fn my_print(&self) {
        println!("This is custom implementation print :(")
    }
}

fn main() {
    let s: String = "This is a test string!".to_owned();
    let bz: Vec<u8> = vec![1, 2, 4, 8, 16];
    s.my_print();
    bz.my_print();
    call_print(&s);
    println!("\n\n");
    
    // ? What will these be?
    &s.my_print();
    (&&&&s).my_print();
    s.as_str().my_print();
    (&&s as &str).my_print();
    <&str>::my_print(&s.as_str());
    (&bz.as_slice()).my_print();
    
    // ? Should this work? Which would it print?
    println!("\n\n");
    MyStruct.my_print();
    MyStruct::my_print(&MyStruct);
    MyPrint::my_print(&MyStruct);
}
