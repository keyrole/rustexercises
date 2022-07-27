mod  sort;
use sort::*;
fn main() {
    let mut vec_i32 = vec![1, 5, 10, 2, 15];
    let mut vec_f32 = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    let mut vec_person = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
        ];
    sort_stable(&mut vec_i32);
    println!("{:?}", vec_i32);
    vec_i32.reverse();
    println!("{:?}", vec_i32);
    sort_unstable(&mut vec_i32);
    println!("{:?}", vec_i32);

    sort_unstable_by_f32(&mut vec_f32);
    println!("{:?}", vec_f32);
    sort_by_struct(&mut vec_person);
    println!("{:?}", vec_person);

}