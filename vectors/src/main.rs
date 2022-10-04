fn main() {

    // Declaring a new vector
    let mut v: Vec<i32> = Vec::new();

    for i in 0..5 {
        v.push(i)
    }

    println!("{:?}", v);

    let third: &i32 = &v[2];
    println!("The fourth element of which is {}", third);

    for i in &mut v {
        *i = *i * 50;
        println!("{i}");
    }
}
