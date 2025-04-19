fn main(){
    let tup: (i32, f64, u8) = (500, 6.48, 1);
    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {}", x, y, z);
    println!("x, y & z are: {}, {}, {}", tup.0, tup.1, tup.2);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}", five_hundred);
    println!("six_point_four: {}", six_point_four);
    println!("one: {}", one);
}
