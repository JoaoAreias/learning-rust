fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    /*  This won't compile because it has a dangling reference
    ---------------------------------------------------------
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
    ---------------------------------------------------------
    */
    
    // No dangling reference here
    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // --------------------
    let str1 = String::from("abccde");
    let str2 = String::from("xyz");
    let result = longest(str1.as_str(), str2.as_str());
    println!("The longest string is {}", result);
}
