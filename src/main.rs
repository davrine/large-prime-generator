// src/main.rs

mod schorr;
fn main() {
    //println!("Hello, world!");
    let numb = schorr::rand_gen();
    println!("{:#?}", numb);
    let binary: Vec<_> = numb.into_iter().map(|x| format!("{:08b}", x)).collect();
    println!("{:#?}", binary);

    let a = schorr::array_to_bigint(&numb);
    println!("{}", a);
    // let binary_string = format!("{:08b}", numb);
    // println!("Binary representation: {}", binary_string);
    schorr::test(&numb);
}
