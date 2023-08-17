#![allow(dead_code, unused_variables)]


struct Point<T, U> {
    x: T,
    y: U 
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    return largest;
}


fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);


    let char_list = vec!['a', 'y', 'c', 'b', 'f', 'n', 'm']; 
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);

    let p1 = Point {x: 1, y: 3};
    let p2 = Point {x: 2.1, y: 3.2};
    let p3 = Point {x: 2, y: 3.2};
}

