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
}
