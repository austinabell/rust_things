#[derive(Debug, Clone)]
pub struct MyStruct {
    my_string: String,
    bytes: Vec<u8>,
}

impl MyStruct {
    /// This will move ownership of these variables into the function
    // * This is preferred constructor in most cases
    pub fn new_owned(bytes: Vec<u8>, my_string: String) -> Self {
        Self { my_string, bytes }
    }
    /// This will pass a reference to the data, allowing a new copy to be generated for the struct
    /// Only use if you ALWAYS need to use the variables used in construction after
    pub fn new_ref(bz: &[u8], m_str: &str) -> Self {
        Self {
            my_string: m_str.to_owned(),
            bytes: bz.to_vec(),
        }
    }
    pub fn update_string(&mut self, u_string: &str) {
        self.my_string = u_string.to_owned();
    }
}

pub fn move_struct(_s: MyStruct) {}
pub fn pass_ref(_s: &MyStruct) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_code() {
        let gen_string: String = "This is a test string".to_owned();
        let gen_bz: Vec<u8> = [1, 3, 5, 6, 2].to_vec();

        let mut my_s: MyStruct = MyStruct::new_owned(gen_bz, gen_string);

        // This struct can be printed only if by reference
        pass_ref(&my_s);
        move_struct(my_s.clone());

        let new_string = "this is the new string";
        // use return struct as mutable reference
        my_s.update_string(&new_string);
        assert_eq!(my_s.my_string, new_string.to_owned());
    }
}
