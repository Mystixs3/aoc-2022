use std::fs;

fn main() {
    let input: String = fs::read_to_string("input.txt").unwrap();

    let mut input_lines = input.lines();

    let mut current: Option<&str>;
    let mut first_elf: u64 = 0;
    let mut second_elf: u64 = 0;
    let mut third_elf: u64 = 0;
    let mut current_elf: u64 = 0;

    loop {
        current = input_lines.next();

        if current == None || current.unwrap() == ""{
            if current_elf > first_elf {
                third_elf = second_elf;
                second_elf = first_elf;
                first_elf = current_elf;
            }
            else if current_elf > second_elf {
                third_elf = second_elf;
                second_elf = current_elf;
            }
            else if current_elf > third_elf {
                third_elf = current_elf;
            }
            current_elf = 0;

            if current == None {
                break;
            }
        }
        else {
            current_elf = current_elf + current.unwrap().parse::<u64>().unwrap();
        };
    }

    let answer: u64 = first_elf + second_elf + third_elf;
    println!("{}", answer);
}