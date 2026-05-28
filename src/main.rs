mod decode_morse;
use decode_morse::decode_morse;

fn main() {
    // decode_morse("")
    // decode_morse("   ")
    // decode_morse("         ")
    println!("decode_morse: {:?}", decode_morse("   "));
}
