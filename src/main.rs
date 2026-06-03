mod human_readable_time;
use human_readable_time::make_readable;

fn main() {
    println!("make_readable: {:?}", make_readable(50));
    println!("make_readable: {:?}", make_readable(60));
    println!("make_readable: {:?}", make_readable(70));
    println!("make_readable: {:?}", make_readable(122));
}
