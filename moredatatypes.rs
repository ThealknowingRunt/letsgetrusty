#![allow(dead_code)]

#[derive(Debug)] // #![drive(debug)] is not a thing why???????
struct Attributes {
    corners: i32,
    r#type: String,
} // apparently type is a keyword so then i have to use r# to not throw an error

#[derive(Debug)]
struct Coords {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    p2: Coords,
    p1: Coords,
    p3: Coords,
    p4: Coords,
}
#[derive(Debug)]
enum Block {
    Name(String),
    Plot,
    Attributes,
    Deaths(i32),
    Age(i32),
}

fn main() {
    let building = Attributes {
        corners: 8,
        r#type: String::from("cube"), // you can't do {8, "cube"}
    };
    let anotherbuilding = Attributes {
        r#type: String::from("cuboid"),
        ..building // coping from another building or rather stealing
    };

    println!("printing what the building looks like: \n{:#?}", building);
    println!(
        "printing another building, wait! are we in a matrix{:#?}",
        anotherbuilding
    );

    let plot = Rectangle {
        p1: Coords { x: 2, y: 3 },
        p2: Coords { x: 9, y: 6 },
        p3: Coords { x: 87, y: 5 },
        p4: Coords { x: 33, y: 44 },
    };

    println!("{:#?}\nyour plot looks ugly!!", plot);
    let name = Block::Name(String::from("fblock"));
    // let _area = Block::Plot(plot);
    println!("here is the fblock ig: {:?}", name);
    let _variable: i32 = 344;
}
