use std::io;

fn main() {
    println!("enter number");
    let mut xs = String::new();
    io::stdin().read_line(&mut xs);
    println!("{}", xs);
}
