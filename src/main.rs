mod likes;
use likes::likes;

fn main() {
    println!("likes: {:?}", likes(&["Alex", "Jacob", "Mark", "Max", "tizio", "caio", "sempronio"]));
}
