fn main() {
    #[derive(Debug)]      // this allows structs to be printed
    struct Structureig();
    println!("I am tryna print this struct: {:?}", Structureig());

    let c = vec![21, 5, 6778]; //for some reason derive(Debug) dosent work with vec

    println!("{:?}", c); //{:?} this will print an ugly c with bracks and all
    println!(r#"this literally prints the thing: "{:?}""#, c); // damn this formatting is amazing
}
