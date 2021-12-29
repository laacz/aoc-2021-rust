use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<i32> = read_file_as_lines(&file_name).iter().map(|l| l.parse().unwrap()).collect();

        let mut increases = 0;
        for i in 1..input.len() {
            increases += if input[i] > input[i - 1] { 1 } else { 0 };
        }

        println!("Part 1 result: {}", increases);

        let windows: Vec<i32> = input.windows(3).map(|w| w.iter().sum()).collect();
        increases = 0;
        for i in 1..windows.len() {
            increases += if windows[i] > windows[i - 1] { 1 } else { 0 };
        }

        println!("Part 2 result: {}", increases);
    }
}
