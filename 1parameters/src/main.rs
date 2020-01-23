/// Example passing reference to piece of data
// ! Immutable reference
// * Used for reading without having to move the data
fn print_bytes(bz: &[u8]) {
    println!("{:02X?}", bz);
}

/// Example of passing mutable reference into function
// ! Mutable reference using heap
// * Used for when a parameter needs to be updated without copying/ cloning
fn append_bytes(vec: &mut Vec<u8>, new_bz: &[u8]) {
    vec.extend_from_slice(new_bz);
}

/// Can mutate but not allow to be dynamically allocated in function
// ! Mutable reference
// * Can also pass a mutable reference to bytes to allow data to be modified but not reallocated
fn blank_first_byte(bz: &mut [u8]) {
    if !bz.is_empty() {
        bz[0] = 0;
    }
}

/// Can pass a struct itself into a function and move it onto that stack frame
// ! Move
// * This is used when the data being passed in isn't or shouldn't be used after
fn move_vec(_v: Vec<u8>) {
    // doesn't matter what happens here
}

/// Can pass back a reference to the data if the lifetime of data in parameters outlives or same
// * lifetimes not needed here, since they will always have same here, just to make point clear
fn first_half<'param: 'ret, 'ret>(v: &'param [u8]) -> &'ret [u8] {
    &v[..v.len() / 2]
}

struct StructWithBytes<'bz> {
    bytes: &'bz mut [u8],
}

// * lifetimes are needed here though because there are two possible lifetimes
fn struct_bytes<'param: 'ret, 'ret>(st: &'param StructWithBytes) -> &'ret [u8] {
    &st.bytes
}

/// Lifetimes must be declared and constrained if more than one possible lifetime
fn choose_bytes<'a: 'c, 'b: 'c, 'c>(b1: &'a [u8], b2: &'b [u8], c: bool) -> &'c [u8] {
    if c {
        b1
    } else {
        b2
    }
}

/// Lifetime does not needs to be defined if multiple references,
/// but not contrained if only one is returned
pub fn print_return<'a>(s: &str, bz: &'a [u8]) -> &'a [u8] {
    println!("{}", s);
    bz
}

/// Can pass back pointer to heap allocated data
fn generate_heap_data() -> Vec<u8> {
    vec![1, 4, 2, 0]
}

fn main() {
    let mut bytes: Vec<u8> = vec![4, 255, 134, 64, 0];
    // Print starting bytes
    print_bytes(&bytes);

    println!("\nAppending [1, 3, 4] to vector");
    let new_data = [1, 3, 4];
    append_bytes(&mut bytes, &new_data);
    print_bytes(&bytes);

    println!("\nBlanking out first byte");
    blank_first_byte(bytes.as_mut_slice());
    print_bytes(&bytes);

    println!("\nPrinting out first half of bytes");
    print_bytes(first_half(&bytes));

    println!("\nPrinting out original bytes from inside struct");
    print_bytes(struct_bytes(&StructWithBytes { bytes: &mut bytes }));

    let b1 = &[1];
    let b2 = &[2];
    let rb = {
        // let b2 = generate_heap_data();
        choose_bytes(b1, b2, true)
    };
    print_bytes(rb);

    println!("\nPrint data allocated in function");
    print_bytes(&generate_heap_data());

    // Try to use move_vec(..)
    move_vec(bytes);
    // * Can't use bytes after this unless it is cloned above
    // print_bytes(&bytes);
}
