fn main() {
    let size: Option<i32> = Some(3);
    let _color = Some(4);
    if let Some(i) = size {
        println!("guess what you are correct{:?}", i);
    }
    if let Some(i) = size {
        println!("color is also correct {:?}", i);
    } else {
        println!("not the one");
    }
}
