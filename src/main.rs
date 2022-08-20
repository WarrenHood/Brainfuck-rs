#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Inc(u8),
    Dec(u8),
    Next(usize),
    Prev(usize),
    LoopStart,
    LoopEnd,
    Print,
    Read
}

use std::io::Read;

use self::Op::*;

fn lex(prog: &str) -> Vec<Op>{
    let mut ops: Vec<Op> = vec![];
    for ch in prog.chars() {
        match ch {
            '>' => {ops.push(Next(1))},
            '<' => {ops.push(Prev(1))},
            '+' => {ops.push(Inc(1))},
            '-' => {ops.push(Dec(1))},
            '[' => {ops.push(LoopStart)},
            ']' => {ops.push(LoopEnd)},
            ',' => {ops.push(Read)},
            '.' => {ops.push(Print)},
            _ => {}
        }
    }
    ops
}

fn optimize(ops: Vec<Op>) -> Vec<Op> {
    let mut new_ops: Vec<Op> = vec![];
    let mut i: usize = 0;
    let mut last_op: Option<&Op> = None;
    let mut add_diff: i16 = 0;
    let mut jmp_diff: isize = 0;
    while i < ops.len() {
        let this_op: &Op = &ops[i];
        let mut this_arithmetic: bool = false;
        let mut this_jump: bool = false;
        let mut last_arithmetic: bool = false;
        let mut last_jump: bool = false;
        if *this_op == Inc(1) || *this_op == Dec(1) {
            this_arithmetic = true;
        }
        else if *this_op == Next(1) || *this_op == Prev(1) {
            this_jump = true;
        }
        match last_op {
            Some(last) => {
                if *last == Inc(1) || *last == Dec(1) {
                    last_arithmetic = true;
                }
                else if *last == Next(1) || *last == Prev(1) {
                    last_jump = true;
                }
            },
            _ => {}
        }

        // Continue with arithmetic stuff
        if this_arithmetic {
            if *this_op == Inc(1) {
                add_diff += 1;
            }
            else if *this_op == Dec(1) {
                add_diff -= 1;
            }

            if last_jump {
                if jmp_diff > 0 {
                    new_ops.push(Next(jmp_diff as usize));
                }
                else if jmp_diff < 0 {
                    new_ops.push(Prev(-jmp_diff as usize));
                }
                jmp_diff = 0;
            }
        }
        else if this_jump { // Continue with jump stuff
            if *this_op == Next(1) {
                jmp_diff += 1;
            }
            else if *this_op == Prev(1) {
                jmp_diff -= 1;
            }

            if last_arithmetic {
                if add_diff > 0 {
                    new_ops.push(Inc(add_diff as u8));
                }
                else if add_diff < 0 {
                    new_ops.push(Dec(-add_diff as u8));
                }
                add_diff = 0;
            }
        }
        else {
            // Well, this is something that isn't a jump or arithmetic op
            
            if last_arithmetic {
                if add_diff > 0 {
                    new_ops.push(Inc(add_diff as u8));
                }
                else if add_diff < 0 {
                    new_ops.push(Dec(-add_diff as u8));
                }
                add_diff = 0;
            }

            if last_jump {
                if jmp_diff > 0 {
                    new_ops.push(Next(jmp_diff as usize));
                }
                else if jmp_diff < 0 {
                    new_ops.push(Prev(-jmp_diff as usize));
                }
                jmp_diff = 0;
            }

            // Just add the next instruction as is
            new_ops.push(*this_op);
        }
        last_op = Some(this_op);
        i += 1;
    }

    if add_diff > 0 {
        new_ops.push(Inc(add_diff as u8));
    }
    else if add_diff < 0 {
        new_ops.push(Dec(-add_diff as u8));
    }
    if jmp_diff > 0 {
        new_ops.push(Next(jmp_diff as usize));
    }
    else if jmp_diff < 0 {
        new_ops.push(Prev(-jmp_diff as usize));
    }

    new_ops
}

fn show_mem(mem_ptr: &mut usize, mem: &mut [u8]) {
    let mem_start = 1500;
    let mem_end = 1550;
    print!("|");
    for i in mem_start..mem_end {
        print!("{:03}|", mem[i] as u8);
    }
    print!("\n ");
    for i in mem_start..mem_end {
        if i == *mem_ptr {
            print!(" ^  ");
        }
        else {
            print!("    ");
        }
    }
    print!("\n");
}

fn show_current_pos(pc: &mut usize, prog: &[Op]) {
    println!("--------------------------------------------------------------");
    let mut pc2 = 0;
    while pc2 < prog.len() {
        if *pc == pc2 {
            print!("* ");
        }
        println!("{:?}", prog[pc2]);
        pc2 += 1;
    }
    println!("--------------------------------------------------------------\n\n");
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
}

fn run(op: &Op, mem: &mut [u8], prog: &[Op], pc: &mut usize, mem_ptr: &mut usize) {
    // For debugging
    // show_mem(mem_ptr, mem);
    // show_current_pos(pc, prog);

    match op {
        Inc(amt) => { mem[*mem_ptr] += amt},
        Dec(amt) => { mem[*mem_ptr] -= amt},
        Next(amt) => { *mem_ptr += amt},
        Prev(amt) => { *mem_ptr -= amt},
        LoopStart => {
            *pc += 1;
            let loop_start = *pc;
            if mem[*mem_ptr] != 0 {
                while mem[*mem_ptr] != 0 {
                    let mut next_op = &prog[*pc];
                    while *next_op != LoopEnd {
                        // Only execute the inner loop code
                        run(next_op, mem, prog, pc, mem_ptr);
                        next_op = &prog[*pc];
                    } 
                    // We have found the end of the loop
                    if mem[*mem_ptr] != 0 {
                        // Go back to the start of the loop
                        *pc = loop_start;
                    }
                }
            }
            else {
                // Advance past the end of the loop
                let mut loop_count = 1;
                loop {
                    let next_op = &prog[*pc];
                    if *next_op == LoopStart {
                        loop_count += 1;
                    }
                    else if *next_op == LoopEnd {
                        loop_count -= 1;
                        if loop_count == 0 {
                            break;
                        }
                    }
                    *pc += 1;
                }
            }
            
        },
        LoopEnd => {
            // This should not happen
            panic!("Unexpected loop end found at {}", *pc);
        },
        Print => { print!("{}", mem[*mem_ptr] as char)},
        Read => {
            loop {
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                if line.trim().len() > 0 {
                    mem[*mem_ptr] = line.trim().chars().nth(0).unwrap() as u8;
                    break;
                }
            }
        }
    }
    *pc += 1;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\tbrainfuck <brainfuck-source-file>");
        std::process::exit(0);
    }
    let mut program_source = String::new();
    std::fs::File::open(&args[1]).expect("name of a brainfuck source file").read_to_string(&mut program_source).unwrap();
    let program = optimize(lex(&program_source));
    let mut mem: [u8; 3000] = [0; 3000];
    let mut pc: usize = 0;
    let mut mem_ptr: usize = 1500;

    while pc < program.len() {
        run(&program[pc], &mut mem, &program, &mut pc, &mut mem_ptr);
    }
    println!("\nDone");
}
