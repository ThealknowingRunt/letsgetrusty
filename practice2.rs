fn main() {
    #[derive(Debug)]
    struct Structureig(); /*
                          even after derive(Debug) it doesnt work ig it needs some other import
                          i did just print it with {} maybe thats why
                          */
    println!("I am tryna print this struct: {:?}", Structureig());

    
    let c = vec![21, 5, 6778]; //for some reason derive(Debug) dosent work with vec
    
    println!("{:?}", c);                                       //{:?} this will print an ugly c with bracks and all
    println!(r#"this literally prints the thing: "{:?}""#, c); // damn this formatting is amazing
}
