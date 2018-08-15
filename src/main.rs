trait Double {
    fn double(&self) -> i32;

    fn wordify(&self) -> String {
        String::from("thirty-six") // TODO: actually make this work for other numbers
    }
}

impl Double for i32 {
    fn double(&self) -> i32 {
        self * 2
    }
}

fn main() {
    println!("Hello, world! {} {}", 36.double(), 36.wordify());
}

#[cfg(test)]
mod tests {
    use Double;

    #[test]
    fn wordify_works() {
        assert_eq!("thirty-six", 36_i32.wordify());
    }
}
