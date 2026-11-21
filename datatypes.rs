#[allow(unused)]
fn main() {
    let s: i32 = 3; // this and
    let a = 3i32; // this
                  // are the same implementation of of an 32 bit signed integer
                  /* the data types that rust has to offer can be
                   * read from docs.
                   */
    let mut defaultvalue = 3; // default integer is i32
                              //defaultvalue = 3.0; // default float is f64

    // by default rust doesn't allow changing value of once initialized variable
    // but mut word can be used to allow that
    let mut num = 34;
    num = 35;
    let num = 35; // even after assigning num 35
                  // we can again assign the same value with let
                  // this is called shadowing
                  /*
                   * along with the basic types we also have
                   * array and tuples
                   * kinda like tuples, set, list from python
                   */
    let saidarray: [i32; 4] = [3, 5, 5, 7]; // [type; length]
    let tup = (3, 5.66, "string", true); // just the classic old tuple
    println!(
        "guess what this substraction is {}- {}= {}(this) or {}(this)",
        2,
        3,
        2u32 - 3,
        2i32 - 3
    ); // thats right u for whole number
    let _m = 233i32;
}
