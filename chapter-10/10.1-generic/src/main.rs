use std::vec;

fn main() {
    println!("hello world");

    let number_list = vec![34, 50 ,25, 100, 65];
    let result = largest2(&number_list);
    println!("the largest is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result= largest2(&char_list);
    println!("the largest is {}",result);
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest2<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}