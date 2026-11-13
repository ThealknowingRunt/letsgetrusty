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
}
/*do also check out format in std::fmt */
