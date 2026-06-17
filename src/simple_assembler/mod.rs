use std::collections::HashMap;

/// We want to create a simple interpreter of assembler which will support the following instructions:
///
/// - mov x y - copies y (either a constant value or the content of a register) into register x
/// - inc x - increases the content of the register x by one
/// - dec x - decreases the content of the register x by one
/// - jnz x y - jumps to an instruction y steps away (positive means forward, negative means backward, y can be a register or a constant), but only if x (a constant or a register) is not zero
///
/// Register names are alphabetical (letters only). Constants are always integers (positive or negative).
///
/// Note: the jnz instruction moves relative to itself. For example, an offset of -1 would continue at the previous instruction, while an offset of 2 would skip over the next instruction.
///
/// The function will take an input list with the sequence of the program instructions and will execute them. The program ends when there are no more instructions to execute, then it returns a dictionary (a table in COBOL) with the contents of the registers.
///
/// Also, every inc/dec/jnz on a register will always be preceeded by a mov on the register first, so you don't need to worry about uninitialized registers.
///
/// Example
/// ```
///     ["mov a 5"; "inc a"; "dec a"; "dec a"; "jnz a -1"; "inc a"]
///     
///     visualized:
///     
///     mov a 5
///     inc a
///     dec a
///     dec a
///     jnz a -1
///     inc a
/// ```
///
/// The above code will:
///
/// set register a to 5,
/// increase its value by 1,
/// decrease its value by 2,
/// then decrease its value until it is zero (jnz a -1 jumps to the previous instruction if a is not zero)
/// and then increase its value by 1, leaving register a at 1
/// So, the function should return:
///
/// `{"a": 1}`
pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut registers: HashMap<String, i64> = HashMap::new();

    let mut pointer: i32 = 0;

    while pointer < program.len().try_into().unwrap() {
        let instruction = program[pointer as usize];

        let command = &instruction[0..3];
        let args = &instruction.split_whitespace().collect::<Vec<_>>()[1..];

        match command {
            "mov" => {
                // args[1] can be a constant or a register
                let value = args[1]
                    .parse::<i64>()
                    .unwrap_or_else(|_| *registers.get(args[1]).unwrap());
                registers.insert(args[0].to_owned(), value);
            }
            "inc" => {
                // increment arg[0] by 1
                registers.entry(args[0].to_owned()).and_modify(|v| {
                    *v += 1;
                });
            }
            "dec" => {
                // decrement arg[0] by 1
                registers.entry(args[0].to_owned()).and_modify(|v| {
                    *v -= 1;
                });
            }
            "jnz" => {
                // jump if not zero
                let mut tmp = 0;
                let value = *registers
                    .get(args[0])
                    .unwrap_or_else(|| {
                        tmp = args[0].parse::<i64>().unwrap();
                        &tmp
                    });
                if value != 0 {
                    // decrease pointer by 1 (counter-increment)
                    pointer -= 1;
                    // add args[1] to pointer
                    let increment = args[1].parse::<i32>().unwrap();
                    pointer += increment;
                }
            }
            _ => (),
        }

        pointer += 1;
    }

    registers
}
