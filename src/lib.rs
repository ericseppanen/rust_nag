//! This is a test library. It's only here so that we can
//! try out some rustfmt changes and clippy lint warnings.


pub struct Foo {
    name: String,
    value: u64,
}

impl Foo {
    pub fn hello(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }
}

pub fn print_things() {
    println!("Hello world");
    println!("Some integers: {} {}", 42u8, 1000u32);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
