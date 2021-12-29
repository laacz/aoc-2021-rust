use aoc2021::{get_arg_or_die, read_file_as_lines};

fn main() {
    if let Some(file_name) = get_arg_or_die(1) {
        let input: Vec<String> = read_file_as_lines(&file_name);
        let mut points: Vec<usize> = Vec::new();

        for line in input {
            let new_line = line.replace(",", " ").replace(" ->", "");
            let v: Vec<usize> = new_line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            points.extend(v);
        }

        println!("Part 1 result: {}", count_overlaps(&points, false));
        println!("Part 2 result: {}", count_overlaps(&points, true));
    }
}

fn count_overlaps(points: &Vec<usize>, diagonals: bool) -> usize {
    let max = points.iter().max().unwrap() + 1;
    let mut area: Vec<Vec<usize>> = vec![vec![0; max]; max];

    for line in points.chunks(4) {
        // You can't do arithmetics on unsigned int's when it is possible that result might be signed.
        let dx = (line[2] as isize - line[0] as isize).signum();
        let dy = (line[3] as isize - line[1] as isize).signum();

        if !diagonals && dx != 0 && dy != 0 {
            continue;
        }

        if let [mut x, mut y] = line[..2] {
            loop {
                area[y][x] += 1;
                if x == line[2] && y == line[3] {
                    break;
                }
                // This also is something I'm dreading. Same signed + unsigned arithmetics.
                // Point being - avoid overflows when usize sees negative values.
                x = if dx.is_negative() { x - dx.abs() as usize } else { x + dx as usize };
                y = if dy.is_negative() { y - dy.abs() as usize } else { y + dy as usize };
            }
        }
    }

    area.iter().map(|row| row.iter().filter(|&&x| x > 1).count()).sum()
}
