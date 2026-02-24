use std::io::Read;
use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("No Input file provided!");
        std::process::exit(1);
    }

    let file_path = &args[1];
    
    let input: Vec<char> = fs::read_to_string(file_path)
    .expect("Should have been able to read the file")
    .chars()
    .collect();


    let size: usize = 30_000;
    let mut cells: Vec<u8> = vec![0; size];

    run(&input, &mut cells);
}

fn run(program: &[char], cells: &mut [u8]) {
    
    let program_length: usize = program.len();
    
    let mut point: usize = 0;
    let mut program_point: usize = 0;
    
    while program_point < program_length {
        let c: char = program[program_point];
    
        match c {
            '>' => point += 1,
            '<' => point -= 1,
            '+' => cells[point] = cells[point].wrapping_add(1),
            '-' => cells[point] = cells[point].wrapping_sub(1),
            '.' => print!("{}", cells[point] as char),
            '[' => {
                if cells[point] == 0 { 
                    let mut bracket_count: i32 = 1;
                    while bracket_count > 0 {
                        program_point += 1;
                        if program[program_point] == '[' {
                            bracket_count += 1;
                        } else if program[program_point] == ']' {
                            bracket_count -= 1;
                        }
                    }
                }
            },
            ']' => {
                if cells[point] != 0 {
                    let mut bracket_count: i32 = 1;
                    while bracket_count > 0 {
                        program_point -= 1;
                        if program[program_point] == ']' {
                            bracket_count += 1;
                        } else if program[program_point] == '[' {
                            bracket_count -= 1;
                        }
                    }
                }
            },
            ',' => {
                let mut input_buffer: [u8; 1] = [0];
                std::io::stdin().read(&mut input_buffer).unwrap();
                cells[point] = input_buffer[0];
            }
    
            _ => ()
        }
    
        program_point += 1;
    }
}