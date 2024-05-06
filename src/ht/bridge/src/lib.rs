#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        fn say_hello();
    }
}

pub fn say_hello() {
    println!("Hello from Rust!");
}
