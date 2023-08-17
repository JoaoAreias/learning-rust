#![allow(dead_code, unused_variables)]
use std::fs::File;
use std::io::ErrorKind;

fn foo() {
    bar();
}

fn bar() {
    baz();
}

fn baz() {
    this_function_crashes_the_program();
}

fn this_function_crashes_the_program(){
    panic!("Crashed");
}


fn main() {
    // this_function_crashes_the_program();
    // foo();    
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating file: {:?}", e), 
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        }
    };
}
