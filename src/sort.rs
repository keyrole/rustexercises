
pub fn sort_stable(vec: &mut Vec<i32>) {
    vec.sort();
}

pub fn sort_unstable(vec: &mut Vec<i32>) {
    vec.sort_unstable();
}

pub fn sort_unstable_by_f32(vec: &mut Vec<f32>) {
    vec.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Person {
    name: String,
    age: u32
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age}
    }
}

pub fn sort_by_struct(vec: &mut Vec<Person>) {
    vec.sort_unstable_by(|a, b| a.age.partial_cmp(&b.age).unwrap());
}