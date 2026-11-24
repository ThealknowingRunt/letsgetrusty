#[derive(Debug)]
#[allow(dead_code)]
enum Btnpos {
    // button position in levels
    Top(u32, u32, u32),
    Left(u32, u32, u32),
    Bottom(u32, u32, u32),
    Right(u32, u32, u32),
    Topleft(u32, u32, u32),
    Topright(u32, u32, u32),
    Bottomright(u32, u32, u32),
    Bottomleft(u32, u32, u32),
}
struct BtnposStruct {
    Top: (u32, u32, u32),
    Left: (u32, u32, u32),
    Bottom: (u32, u32, u32),
    Right: (u32, u32, u32),
    Topleft: (u32, u32, u32),
    Topright: (u32, u32, u32),
    Bottomright: (u32, u32, u32),
    Bottomleft: (u32, u32, u32),
}

fn main() {
    let names = vec!["personA", "personB", "personC"];
    let ages = (3, 6, 4, 9, 8, 5, 7);
    for name in names.iter() {
        let _result = match name {
            &"personA" => println!("first subject aye"), // with the .iter only &"presonA" is allowed you know why
            // to change it one must betake harehest of pusnishments
            // just kidding, u need to use into_iter() and mut_iter for mutable ref
            _ => println!("no thanks we will be away now"),
        };
    }
    //next is match
    let result = match ages {
        (1, ..) => String::from("first is first none the less"),
        (3, ..) | (4, ..) => "upon revaluation it seems to be correct".to_string(),
        (.., 7) => "second condition is also true".to_string(), // for some reason the (.., 4, ..) is not allowed
        // and also if 2 or more conditions are true then first one will be allowed passage
        _ => "none".to_string(),
    };
    // we can however do this
    /* if ages.contains(&3) {  // this doesn't work sed
        println!("yes ");
    } */
    println!("result: {}", result);
    let anarray = [2, 4, 6, 8, 10];
    match anarray {
        [2, snd, ..] => println!("arr[0] , arr[1]({}) and the rest is not known ", snd),
        [.., 8, last] => println!(
            "rest of the array , second last is 8 and the last is {}",
            last
        ),
        [head @ .., 6, _, last] => println!(
            "the array is simply {:?}+6 and also {} oops second last was lost",
            head, last
        ), // i think mostly just the @ part is new and maybe the
        // rest logic is same in whole language
        _ => println!("nothing really thats it "),
    }
    let enter = Btnpos::Right(0, 1, 0);
    let esc = BtnposStruct(Topleft(1, 0, 0));
    println!("position of enter is {:?}", enter);

    match enter {
        // this is how you use a gaurd
        // to set an additional conditional
        // this is something that took a good long try to realise
        // if not for a worthy example where would i be
        Btnpos::Right(l1, l2, l3) if (l1, l2, l3) == (0, 1, 0) => {
            println!("enter is at {}, {}, {}", l1, l2, l3)
        }
        Btnpos::Right(l1, l2, l3) if (l1, l2, l3) == (1, 0, 0) => {
            println!("enter is at {}, {}, {}", l1, l2, l3)
        }
        _ => println!("no order is accepted"),
    }
}
