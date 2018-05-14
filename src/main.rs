trait Double {
    fn double(&self) -> i32;
}

impl Double for i32 {
    fn double(&self) -> i32 {
        self * 2
    }
}

fn main() {
    println!("Hello, world! {}", 36.double());
}
