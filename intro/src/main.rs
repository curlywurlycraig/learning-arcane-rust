/// This program shows Undefined Behaviour by reading pointer offsets
/// beyond the allocated memory of the object (string, in this case).
fn main() {
    let a: &str = "abc";
    let aa: &str = "def";
    let b: *const u8 = a.as_ptr();

    unsafe {
	println!("{}", *b.offset(0) as char);
	println!("{}", *b.offset(1) as char);
	println!("{}", *b.offset(2) as char);

	// Whoops! Probably (but maybe not, it is UB) prints "d"
	println!("{}", *b.offset(3) as char);
    }
}
