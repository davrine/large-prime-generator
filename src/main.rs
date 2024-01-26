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

    println!("{}", schorr::miller_rabin(&a, 128));
    // loop {
    //     let a = &schorr::array_to_bigint(&schorr::rand_gen());
    //     let is_prime = schorr::miller_rabin(&a, 128);
    //     println!("{}: {}", is_prime, a);
    //     if is_prime {
    //         break;
    //     }
    // }
    schorr::gen_prime();
    // let binary_string = format!("{:08b}", numb);
    // println!("Binary representation: {}", binary_string);
}
