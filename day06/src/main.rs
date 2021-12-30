use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);

        let fishes = input[0].split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>();

        let mut counts: Vec<u64> = vec![0; 9];
        for &fish in &fishes {
            counts[fish] += 1;
        }

        for day in 1..=256 {
            let mut new_counts: Vec<u64> = vec![0; 9];
            for timer in 0..=8 {
                if timer == 0 {
                    new_counts[8] = counts[timer];
                    new_counts[6] += counts[timer];
                } else {
                    new_counts[timer - 1] += counts[timer];
                }
            }
            counts = new_counts;
            if day == 80 {
                println!("Part 1 result: {}", counts.iter().sum::<u64>());
            } else if day == 256 {
                println!("Part 2 result: {}", counts.iter().sum::<u64>());
            }
        };
    }
}