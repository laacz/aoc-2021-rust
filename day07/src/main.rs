use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);

        let positions = input[0]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i32>>();

        let min = *positions.iter().min().unwrap();
        let max = *positions.iter().max().unwrap();

        let mut costs1: Vec<i32> = vec![0; (max - min + 1) as usize];
        let mut costs2: Vec<i32> = vec![0; (max - min + 1) as usize];

        for position in min..=max {
            costs1[position as usize] = cost1(&position, &positions);
            costs2[position as usize] = cost2(&position, &positions);
        }

        println!("Part1: {:?}", costs1.iter().min().unwrap());
        println!("Part2: {:?}", costs2.iter().min().unwrap());
    }
}

fn cost1(position: &i32, positions: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for p in positions.iter() {
        sum += (p - position).abs();
    }
    sum
}

fn cost2(position: &i32, positions: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for p in positions.iter() {
        let distance = (position - p).abs();
        if distance > 1 {
            sum += distance * (distance + 1) / 2 as i32;
        } else {
            sum += distance;
        }
    }
    sum
}
