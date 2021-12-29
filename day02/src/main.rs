use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);

        let mut position = 0;
        let mut depth = 0;
        let mut depth2 = 0;
        let mut aim = 0;

        for line in input {
            let tmp: Vec<&str> = line.split(" ").collect();
            let direction = tmp[0];
            let value: i32 = tmp[1].parse().unwrap();

            position += match direction {
                "forward" => value,
                _ => 0,
            };

            depth += match direction {
                "down" => value,
                "up" => -value,
                _ => 0,
            };

            depth2 += match direction {
                "forward" => value * aim,
                _ => 0,
            };

            aim += match direction {
                "down" => value,
                "up" => -value,
                _ => 0,
            };
        }

        println!("Part 1 result: {}", position * depth);
        println!("Part 2 result: {}", position * depth2);
    }
}
