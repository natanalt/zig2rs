mod parser;
mod zig_types;

#[allow(unused_imports)]
use parser::*;
#[allow(unused_imports)]
use zig_types::*;

unsafe fn print_string(ptr: *const u8, len: usize) {
    let slice = std::slice::from_raw_parts(ptr, len);
    let string = std::str::from_utf8(slice).unwrap();
    println!("{}", string);
}

zig2rs! {
    fn hello_world(times: usize) void {
        const message = "this is a crime";
        var i: usize = 0;
        while (i < times) : (i += 1) {
            print_string(message.ptr, message.len);
        }
    }
}

fn main() {
    unsafe {
        hello_world(3);
    }
}
