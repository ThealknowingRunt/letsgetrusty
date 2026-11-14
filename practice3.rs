use std::fmt;

fn main() {
    #[allow(dead_code)]
    struct Structure(i32, i32);
    impl fmt::Display for Structure {
        // all this to just make it so that our custom data_type gets printed
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // this has some really long borrow check thing and really annoying
            write!(f, "{}", self.0) // this prints all
        }
    }
    let output = Structure(3, 5);
    println!("{}", output);
}
