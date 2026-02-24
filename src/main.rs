fn main() {

    let size: usize = 30_000;
    let mut cells: Vec<u8> = vec![0; size];

    // let input: Vec<char> = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.>>>++++++++[<++++>-]<.>>>++++++++++[<+++++++++>-]<---.<<<<.+++.------.--------.>>+.>++++++++++.".chars().collect();
    let input: Vec<char> = "++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++.".chars().collect();
    let input_length: usize = input.len();

    let mut point: usize = 0;
    let mut input_point: usize = 0;
    while input_point < input_length {
        let c: char = input[input_point];

        match c {
            '>' => point += 1,
            '<' => point -= 1,
            '+' => cells[point] = cells[point].wrapping_add(1),
            '-' => cells[point] = cells[point].wrapping_sub(1),
            '.' => print!("{}", cells[point] as char),
            _ => ()
        }

        input_point += 1
    }
}
