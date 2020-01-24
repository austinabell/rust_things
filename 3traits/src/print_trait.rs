use std::fmt;

pub trait MyPrint: fmt::Debug {
    fn my_print(&self) {
        println!("Default:\t\t {:?}", self);
    }
}

impl MyPrint for String {}
impl MyPrint for &str {
    fn my_print(&self) {
        println!("This is a &str:\t\t {}", self);
    }
}
impl MyPrint for str {
    fn my_print(&self) {
        println!("This is a str:\t\t {}", self);
    }
}

impl MyPrint for Vec<u8> {}
impl MyPrint for [u8] {}
impl MyPrint for &[u8] {
    fn my_print(&self) {
        println!("This is a &[u8]:\t {:02X?}", self);
    }
}

pub trait MyOtherTrait {}

impl<T> MyPrint for T
where
    T: fmt::Debug + MyOtherTrait,
{
    fn my_print(&self) {
        println!("I AM MY OTHER TRAIT!");
    }
}
