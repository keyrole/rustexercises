use rand::prelude::*;
mod common;
mod point;
fn main() {

    if random() { // generates a boolean
        // Try printing a random unicode code point (probably a bad idea)!
        println!("char: {}", random::<char>());
    }
    
    let mut rng = thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    println!("{}", y);
    
    let mut nums: Vec<i32> = (1..100).collect();
    println!("{}", nums.choose(&mut rng).unwrap());
    println!("{}", nums.choose_weighted(&mut rng, |&x| x).unwrap());
    nums.shuffle(&mut rng);
    println!("{:?}", nums);

    let distr = rand::distributions::Uniform::new_inclusive(1, 100);
    let mut nums1 = [0i32; 3];
    for x in &mut nums1 {
        *x = rng.sample(distr);
    }
    println!("Some numbers: {:?}", nums1);

    let arrows_iter = "➡⬈⬆⬉⬅⬋⬇⬊".chars();
    println!("Lets go in this direction: {}", arrows_iter.choose(&mut rng).unwrap());

    if rng.gen() { // random bool
        let x: f64 = rng.gen(); // random number in range [0, 1)
        let y = rng.gen_range(-10.0..10.0);
        println!("x is: {}", x);
        println!("y is: {}", y);
    }

    println!("Die roll: {}", rng.gen_range(1..=6));
    println!("Number from 0 to 9: {}", rng.gen_range(0..10));

    let many: (
        (),
        (usize, isize, Option<(u32, (bool,))>),
        (u8, i8, u16, i16, u32, i32, u64, i64),
        (f32, (f64, (f64,))),
    ) = random();

    println!("{:?}", many);
    println!("{}", common::gen_password());
    println!("{}", common::gen_alphanumeric(40));
    println!("{:?}", rng.gen::<point::Point>());
}