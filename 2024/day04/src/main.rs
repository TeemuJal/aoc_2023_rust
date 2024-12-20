use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input2.txt").unwrap();
    let lines: Vec<&str> = input.lines().collect();
    part_one_xmas_count(&lines);
    part_two_xmas_count(&lines);
}

fn part_two_xmas_count(lines: &Vec<&str>) {
    let line_len = lines[0].len();
    let mut xmas_count = 0;
    for line_idx in 1..lines.len() - 1 {
        let line = lines[line_idx];
        let prev_line = lines[line_idx - 1];
        let next_line = lines[line_idx + 1];
        for hor_idx in 1..line_len - 1 {
            if line.chars().nth(hor_idx).unwrap() != 'A' {
                continue;
            }
            let top_left = prev_line.chars().nth(hor_idx - 1).unwrap();
            let top_right = prev_line.chars().nth(hor_idx + 1).unwrap();
            let bot_left = next_line.chars().nth(hor_idx - 1).unwrap();
            let bot_right = next_line.chars().nth(hor_idx + 1).unwrap();
            let diagonal_1_matches = (top_left == 'M' && bot_right == 'S') || (top_left == 'S' && bot_right == 'M');
            let diagonal_2_matches = (top_right == 'M' && bot_left == 'S') || (top_right == 'S' && bot_left == 'M');
            if diagonal_1_matches && diagonal_2_matches {
                xmas_count += 1;
            }
        }
    }
    println!("X-MAS appears {xmas_count} times");
}

fn part_one_xmas_count(lines: &Vec<&str>) {
    let line_len = lines[0].len();
    let mut search_strings: Vec<String> = vec![];
    // Horizontals
    for line in lines {
        search_strings.push(line.to_string());
        search_strings.push(line.chars().rev().collect());
    }
    // Verticals
    for idx in 0..lines.len() {
        let vert_str: String = lines
            .iter()
            .map(|line| line.chars().nth(idx).unwrap())
            .collect();
        search_strings.push(vert_str.clone());
        search_strings.push(vert_str.chars().rev().collect());
    }
    // Descending diagonals
    for idx in (0..lines.len()).rev() {
        let mut line_idx = 0;
        let mut str = String::new();
        for hor_idx in idx..line_len {
            let char = lines[line_idx].chars().nth(hor_idx).unwrap();
            str.push(char);
            line_idx += 1;
        }
        search_strings.push(str.clone());
        search_strings.push(str.chars().rev().collect());
    }
    for idx in 1..lines.len() {
        let mut line_idx = idx;
        let mut str = String::new();
        for hor_idx in 0..line_len {
            if let Some(line) = lines.get(line_idx) {
                let char = line.chars().nth(hor_idx).unwrap();
                str.push(char);
                line_idx += 1;
            } else {
                break;
            }
        }
        search_strings.push(str.clone());
        search_strings.push(str.chars().rev().collect());
    }
    // Ascending diagonals
    for idx in 0..lines.len() {
        let mut line_idx = idx;
        let mut str = String::new();
        for hor_idx in 0..line_len {
            let chr = lines[line_idx].chars().nth(hor_idx).unwrap();
            str.push(chr);
            if let Some(new_line_idx) = line_idx.checked_sub(1) {
                line_idx = new_line_idx
            } else {
                break;
            }
        }
        search_strings.push(str.clone());
        search_strings.push(str.chars().rev().collect());
    }
    for idx in 1..line_len {
        let mut line_idx = lines.len() - 1;
        let mut str = String::new();
        for hor_idx in idx..line_len {
            let chr = lines[line_idx].chars().nth(hor_idx).unwrap();
            str.push(chr);
            if let Some(new_line_idx) = line_idx.checked_sub(1) {
                line_idx = new_line_idx
            } else {
                break;
            }
        }
        search_strings.push(str.clone());
        search_strings.push(str.chars().rev().collect());
    }
    let xmas_count = search_strings
        .iter()
        .flat_map(|str| str.matches("XMAS").collect::<Vec<_>>())
        .count();
    println!("XMAS appears {xmas_count} times");
}
