fn main(){
    let tup: (i32, f64, u8) = (500, 5.4, 1);

    let (x, y, z) = tup;

    let arr = [3; 5];

    println!("The value of y is: {}", arr[1]);
}