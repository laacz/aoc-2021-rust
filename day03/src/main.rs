use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);

        // Part 1
        let len = input[0].len();
        let mut bit_counts: Vec<[i32; 2]> = vec![[0, 0]; len];
        let mut gamma: u32 = 0;
        let mut epsilon: u32 = 0;

        for line in input.iter() {
            for (char_no, char) in line.chars().enumerate() {
                bit_counts[char_no][char.to_digit(10).unwrap() as usize] += 1;
            }
        }

        for i in 0..len {
            let (most_common, _) = count_bits_at_pos(&input, i);
            gamma += most_common * (2 as u32).pow((len - i - 1) as u32);
            epsilon += ((most_common != 1) as u32) * (2 as u32).pow((len - i - 1) as u32);
        }

        // Part 2
        let mut o2_list = input.clone();
        let mut co2_list = input.clone();

        for i in 0..len {
            if o2_list.len() > 1 {
                let (most_common, _) = count_bits_at_pos(&o2_list, i);
                o2_list.retain(|line| line.chars().nth(i).unwrap().to_digit(10).unwrap() as u32 == most_common);
            }
            if co2_list.len() > 1 {
                let (_, least_common) = count_bits_at_pos(&co2_list, i);
                co2_list.retain(|line| line.chars().nth(i).unwrap().to_digit(10).unwrap() as u32 == least_common);
            }
        }

        let o2 = isize::from_str_radix(o2_list[0].as_str(), 2).unwrap();
        let co2 = isize::from_str_radix(co2_list[0].as_str(), 2).unwrap();

        println!("Part 1 result is {}; γ * ε => {} * {}", gamma * epsilon, gamma, epsilon);
        println!("Part 2 result is {}; O₂ * CO₂ => {} * {}", o2 * co2, o2, co2);
    }
}

fn count_bits_at_pos(list: &Vec<String>, pos: usize) -> (u32, u32) {
    let mut counts = vec![0, 0];
    for line in list {
        counts[line
            .chars()
            .nth(pos)
            .unwrap()
            .to_digit(10)
            .unwrap() as usize] += 1;
    }
    let mcb = if counts[0] > counts[1] { 0 } else { 1 };

    (mcb as u32, (mcb != 1) as u32)
}
