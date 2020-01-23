#[derive(Clone, Default, Debug, PartialEq)]
pub struct OtherStruct {
    valid: bool,
}

impl OtherStruct {
    pub fn get(&self) -> &bool {
        &self.valid
    }
    pub fn set(&mut self, valid: bool) {
        self.valid = valid;
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct MyStruct {
    val: u8,
    other: OtherStruct,
}

impl MyStruct {
    pub fn new(val: u8) -> Self {
        Self {
            val,
            // way to default rest of fields (okay back to the point now)
            ..Default::default()
        }
    }

    // ! Immutable getters
    pub fn val(&self) -> &u8 {
        // * pointer allocates more memory than u8 so should probably return copy instead
        &self.val
    }
    pub fn other(&self) -> &OtherStruct {
        &self.other
    }

    // ! Mutable references
    pub fn val_mut(&mut self) -> &mut u8 {
        &mut self.val
    }
    pub fn other_mut(&mut self) -> &mut OtherStruct {
        &mut self.other
    }

    // ! Copied/ cloned return
    pub fn val_copy(&self) -> u8 {
        // * Implicit copy since it is implemented for u8
        self.val
    }
    pub fn other_cloned(&self) -> OtherStruct {
        self.other.clone()
    }
}

pub fn run_example() {
    let mut my_s = MyStruct::new(1);
    assert_eq!(my_s.other(), &OtherStruct { valid: false });
    assert_eq!(my_s.val(), &1);

    let other_clone = my_s.other_cloned();
    // other_clone.set(true);
    // Must get mutable reference to other to set it
    my_s.other_mut().set(true);

    // Other was cloned, so update to it did not affect it
    assert_eq!(other_clone.valid, false);
    assert_eq!(my_s.other().get(), &true);

    // * Can't borrow the same struct as mutable and immutable
    // mut_imut_function(my_s.val_mut(), my_s.other());
}

pub fn mut_imut_function(_value: &mut u8, _other: &OtherStruct) {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_params() {
        run_example();
    }
}
