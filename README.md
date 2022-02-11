# ✨ *zig2rs* ✨
Have you ever wanted to use two of the objectively best programming languages out there - Rust and Zig - at the same time? Has linking applications through C ABI tired you? Well then look no further, as my solution will revolutionise the world of software development!

## Why?
![Ferris the Crab and Zero the Ziguana cuddling on a raft](truelove.png)

## How?
It's a small macro you can use:
```rust
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
```
You can't actually use any good features Zig has going for itself, but shhh be quiet. This example is about everything this thing can do, and anything else will anger the compiler.

## Who?
hi, [I](https://github.com/natanalt) did this medium effort shitpost

## What's next?
Someday I may write a proper compiler using procedural macros instead of `macro_rules!` which would allow for much greater flexibility, lack of which is why this macro is so limited
