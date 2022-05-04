use std::io;

fn main(){
    let a = [1,2,3,4,5];

    println!("Please enter an array index");

    let mut idx = String::new();

    io::stdin()
        .read_line(&mut idx)
        .expect("Failed to read line");

    let idx: usize = idx
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let el = a[idx];

    println!("The val of el at idx {} is: {}", idx, el);

    another_fn(5, 'x');
}

fn another_fn(x: i32, unit_label: char){
    println!("Another {} in {}", x, unit_label);
}