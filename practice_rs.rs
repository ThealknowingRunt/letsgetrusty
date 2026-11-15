use std::fmt;

fn main() {
    println!(
        "{}hello\n this is just one variable being called once\n-------------------",
        3
    );
    println!(
        "{0}first one, {1}second one,{0}first one,{0}first one\n2 variable multiple times\n-------------------",
        3, "2nd"
    );
    println!(
        "{aword}this prints a word\nthis is {aword}, {aword} the same variable\n-------------------",
        aword = "idk"
    );
    println!(
        "a number with some spaces: {n:>3}\n-------------------",
        n = 3
    );
    let _n = 3; // or we can use #[allow(dead_code)]
                //this also means you cant use n but you can use _n
    println!("printing n: {}", _n);
    // println!("printing n: {}", n); this shows error

    #[derive(Debug)] // this allows structs to be printed
    struct Structureig();

    println!("I am tryna print this struct: {:?}", Structureig());

    let c = vec![21, 5, 6778]; //for some reason derive(Debug) dosent work with vec

    println!("{:?}", c); //{:?} this will print an ugly c with bracks and all
    println!(r#"this literally prints the thing: "{:?}""#, c); // damn this formatting is amazing

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
/*do also check out format!() in std::fmt */
