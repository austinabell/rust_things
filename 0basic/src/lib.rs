#[derive(Debug, Clone)]
pub struct MyStruct {
    my_string: String,
    bytes: Vec<u8>,
}

impl MyStruct {
    // * This is preferred constructor in most cases
    pub fn new_owned(bytes: Vec<u8>, my_string: String) -> Self {
        Self { my_string, bytes }
    }
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

fn move_struct(_s: MyStruct) {
    // Doesn't matter
}
fn pass_ref(_s: &MyStruct) {
    // Doesn't matter
}

pub fn generate_new_ref() -> MyStruct {
    let gen_string: String = "This is a test string".to_owned();
    let gen_bz: [u8; 5] = [1, 3, 5, 6, 2];
    // let gen_bz: Vec<u8> = [1, 3, 5, 6, 2].to_vec();

    let mut return_struct: MyStruct = MyStruct::new_ref(&gen_bz, &gen_string);

    // This struct can be printed only if by reference
    pass_ref(&return_struct);
    move_struct(return_struct.clone());

    let new_string = "this is the new string";

    // use return struct as mutable reference
    return_struct.update_string(&new_string);
    assert_eq!(return_struct.my_string, new_string.to_owned());

    // Return that updated struct
    return_struct
}

pub fn generate_new_move() -> MyStruct {
    // Must specifically be the types passed in and
    let gen_string: String = "This is a test string".to_owned();
    let gen_bz: Vec<u8> = [1, 3, 5, 6, 2].to_vec();

    let return_struct: MyStruct = MyStruct::new_owned(gen_bz, gen_string);

    // ! Can't use gen_bz here because it was moved in previous step, must clone if
    // println!("{:?}", gen_bz);

    return_struct
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_code() {
        let _s: MyStruct = generate_new_ref();
        let _s2: MyStruct = generate_new_move();
    }
}
