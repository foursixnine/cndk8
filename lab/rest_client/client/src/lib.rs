#![allow(unused_variables, dead_code)]
fn main() {
    #[no_mangle]
    pub extern "C" fn hello_world() {
        println!("Hello World! FROM RUST BABY!!!");
    }
}
