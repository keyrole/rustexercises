mod linklist;
use linklist::Linklist;

fn main() {
    println!("hi, good");
    let mut ll = Linklist::new(1);
    ll.push(2);
    ll.push(3);
    ll.push(4);
    println!("{:?}", ll);
    let mut p = ll.get_top();
    while let Some(n) = p {
        println!("Data:{}", n.get_data());
        p = n.get_next();
    }
}