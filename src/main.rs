mod simple_assembler;
use crate::simple_assembler::simple_assembler;

fn main() {    
    println!("simple_assembler: {:?}", simple_assembler(vec![
            "mov c 12",
            "mov b 0",
            "mov a 200",
            "dec a",
            "inc b",
            "jnz a -2",
            "dec c",
            "mov a b",
            "jnz c -5",
            "jnz 0 1",
            "mov c a",
        ]));
}
