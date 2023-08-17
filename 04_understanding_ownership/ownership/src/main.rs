fn main() {

    { // begins scope, s is not valid yet
        let s = String::from("Hello, world!"); // s is declared, memory is allocated
        // do stuff with s 
    } // scope is over, s is no longer valid, memory is deallocated

    let x = 5;
    let y = x; // Copy (copy traint)

    let s1 = String::from("Hello");
    let s2 = s1; // Move, not shallow copy
    let s3 = s2.clone(); // More expansive than moving

    takes_ownership(s2);
    // println!("{}", s2)
    // Cannot run the line above as the function took ownership of the variable

    makes_copy(x);
    println!("X: {}", x); // This code works as integers are copied

    let s4 = takes_and_gives_back(s3);
    println!("S4: {}", s4); // Ownership is returnet to s4

    reference_string(&s4); // References don't take ownership of the variable

    let mut s5 = String::from("Hello");
    change(&mut s5);
    println!("S5: {}", s5);
    /* 
    Rules of references
    1. At any given time, you can have either one mutable reference or 
       any number of immutable references.
    2. References must always be valid 
    */


    // Slices
    let hello = &s5[..5];
    let world = &s5[7..];
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string)
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn reference_string(some_string: &String) {
    println!("{}", some_string);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn makes_copy(some_intger: i32) {
    println!("{}", some_intger)
}