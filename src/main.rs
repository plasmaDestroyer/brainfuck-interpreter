use std::io::Read;
use std::env;
use std::fs;

const CELL_SIZE: usize = 30_000;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        eprintln!("No Input file provided!");
        std::process::exit(1);
    }

    let file_path = &args[1];
    let input: Vec<char> = match fs::read_to_string(file_path) {
        Ok(content) => content.chars().collect(),
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", file_path, e);
            std::process::exit(1);
        }
    };


    let mut cells: Vec<u8> = vec![0; CELL_SIZE];

    if let Err(e) = run(&input, &mut cells) {
        eprintln!("{}", e);
        std::process::exit(1);
    };
}

fn run(program: &[char], cells: &mut [u8]) -> Result<(), String> {
    
    let program_length: usize = program.len();
    
    let mut point: usize = 0;
    let mut program_point: usize = 0;
    
    while program_point < program_length {
        let c: char = program[program_point];
    
        match c {
            '>' => {
                if point + 1 == CELL_SIZE {
                    return Err("Cell Index exceeded the Maximum Bounds!".to_string())
                }

                point += 1;
            },
            '<' => {
                if point == 0 {
                    return Err("Cell Index became less than Zero!".to_string())
                }

                point -= 1;
            },
            '+' => cells[point] = cells[point].wrapping_add(1),
            '-' => cells[point] = cells[point].wrapping_sub(1),
            '.' => print!("{}", cells[point] as char),
            '[' => {
                if cells[point] == 0 { 
                    let mut bracket_count: i32 = 1;
                    while bracket_count > 0 {
                        if program_point + 1 == program_length {
                            return Err("Brackets are Mismatched in the input!".to_string())
                        }
                
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
                        
                        if program_point == 0 {
                            return Err("Brackets are Mismatched in the input!".to_string())
                            }

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
                match std::io::stdin().read(&mut input_buffer) {
                    Ok(_) => cells[point] = input_buffer[0],
                    Err(e) => return Err(format!("Failed to read the Input: {e}"))
                }
            },
    
            _ => ()
        }
    
        program_point += 1;
    }

    Ok(())
}