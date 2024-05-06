#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Greeter;

        #[swift_bridge(init)]
        fn new(name: &str) -> Greeter;

        #[swift_bridge(swift_name = "sayHello")]
        fn say_hello(&self);
    }
}

pub struct Greeter {
    name: String,
}

impl Greeter {
    pub fn new(name: &str) -> Self {
        Self { name: name.into() }
    }

    pub fn say_hello(&self) {
        println!("Hello from Rust, {}!", self.name);
    }
}
