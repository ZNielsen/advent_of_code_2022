use std::collections::{HashMap, HashSet, BinaryHeap};

fn main() {
    day_1();
    day_2();
    day_3();
    day_4();
    day_5();
}

fn day_5() {
    let filename = String::from("input_files/day5");
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut input = contents.split("\n\n");
    // Parse starting input
    let mut stack_vec_9000 = parse_initial_setup(input.next().unwrap());
    let mut stack_vec_9001 = stack_vec_9000.clone();
    // Do each line operation
    let cmds = input.next().unwrap();
    for line in cmds.split("\n") {
        if line.is_empty() { continue; }
        let mut line_itr = line.split(" ");
        let num    = usize::from_str_radix(line_itr.nth(1).unwrap(), 10).unwrap();
        let source = usize::from_str_radix(line_itr.nth(1).unwrap(), 10).unwrap();
        let dest   = usize::from_str_radix(line_itr.nth(1).unwrap(), 10).unwrap();

        // Simulate CrateMover 9000
        for _ in 0..num {
            let tmp = stack_vec_9000[source-1].pop().unwrap();
            stack_vec_9000[dest-1].push(tmp);
        }

        // Simulate CrateMover 9001
        let mut tmp = Vec::new();
        for _ in 0..num {
            tmp.push(stack_vec_9001[source-1].pop().unwrap());
        }
        for item in tmp.into_iter().rev() {
            stack_vec_9001[dest-1].push(item);
        }
    }

    let mut result_string_9000 = String::new();
    let mut result_string_9001 = String::new();
    for mut stack in stack_vec_9000 {
        result_string_9000.push_str(&stack.pop().unwrap());
    }
    for mut stack in stack_vec_9001 {
        result_string_9001.push_str(&stack.pop().unwrap());
    }

    println!("Day 5");
    println!("Top Boxes 9000: {}", result_string_9000);
    println!("Top Boxes 9001: {}", result_string_9001);
    println!("");
}
fn parse_initial_setup(input: &str) -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::new();
    for _ in 0..9 {
        data.push(Vec::new());
    }

    for line in input.rsplit("\n") {
        let mut itr = line.chars();
        let ch = itr.nth(1).unwrap();
        if ch.is_digit(10) { continue; } // Skip this, it's the label line
        let mut ch = ch.to_string();
        for idx in 0..9 {
            if ch != " " {
                data[idx].push(ch);
            }
            ch = itr.nth(3).unwrap_or(' ').to_string();
        }
    }

    data
}

fn day_4() {
    let filename = String::from("input_files/day4");
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut full_overlaps = 0;
    let mut partial_overlaps = 0;
    for pairs in contents.split("\n") {
        if pairs.is_empty() { continue; }
        let mut itr = pairs.split(",");
        let elf1 = itr.next().unwrap();
        let elf2 = itr.next().unwrap();

        let mut itr = elf1.split("-");
        let elf1_bot = u32::from_str_radix(itr.next().unwrap(), 10).unwrap();
        let elf1_top = u32::from_str_radix(itr.next().unwrap(), 10).unwrap();
        let mut itr = elf2.split("-");
        let elf2_bot = u32::from_str_radix(itr.next().unwrap(), 10).unwrap();
        let elf2_top = u32::from_str_radix(itr.next().unwrap(), 10).unwrap();

        // Full overlap
        let mut does_overlap = false;
        if elf1_bot <= elf2_bot && elf1_top >= elf2_top {
            does_overlap = true;
        }
        if elf2_bot <= elf1_bot && elf2_top >= elf1_top {
            does_overlap = true;
        }
        if does_overlap {
            full_overlaps += 1;
        }

        // Part overlap
        let mut does_overlap = true;
        if elf1_bot > elf2_top {
            does_overlap = false;
        }
        if elf1_top < elf2_bot {
            does_overlap = false;
        }
        if does_overlap {
            partial_overlaps += 1;
        }
    }

    println!("Day 4");
    println!("Number of full overlaps: {}", full_overlaps);
    println!("Number of part overlaps: {}", partial_overlaps);
    println!("");
}

fn get_prio(ch: char) -> u32 {
    // 'a' is 97, 'A' is 65
    let num = ch as u32;
    match num {
        num if num >= 97 && num <= 97+26 => num - 96,
        num if num >= 65 && num <= 65+26 => num - 64 + 26,
        _ => panic!("yo, something is wrong")
    }
}
fn day_3() {
    let filename = String::from("input_files/day3");
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut total_priority = 0;
    let mut badge_priority = 0;
    let mut elf_group = Vec::<String>::new();
    for bag in contents.split("\n") {
        if bag.is_empty() { continue; }

        elf_group.push(String::from(bag));
        if elf_group.len() == 3 {
            let mut first_elf = HashSet::<char>::new();
            for ch in elf_group[0].chars() {
                first_elf.insert(ch);
            }
            let mut first_two_common = HashSet::<char>::new();
            for ch in elf_group[1].chars() {
                if first_elf.contains(&ch) {
                    first_two_common.insert(ch);
                }
            }
            for ch in elf_group[2].chars() {
                if first_two_common.contains(&ch) {
                    badge_priority += get_prio(ch);
                    break;
                }
            }
            elf_group.clear();
        }

        let left = String::from(&bag[..bag.len()/2]);
        let right = String::from(&bag[bag.len()/2..]);
        let mut set = HashSet::<char>::new();
        for ch in left.chars() {
            set.insert(ch);
        }

        for ch in right.chars() {
            if set.contains(&ch) {
                total_priority += get_prio(ch);
                break;
            }
        }
    }

    println!("Day 3");
    println!("Total Priority: {}", total_priority);
    println!("Badge Priority: {}", badge_priority);
    println!("");
}

#[derive(Copy, Clone)]
enum RPSThrow {
    Rock = 1,
    Paper = 2,
    Scizzors = 3,
}
#[derive(Copy, Clone)]
enum RPSResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
fn day_2() {
    let points_table_throws = HashMap::<&str, u64>::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let points_table_results = HashMap::<&str, u64>::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);
    let throw_table = HashMap::<&str, RPSResult>::from([
        ("A X", RPSResult::Draw),
        ("A Y", RPSResult::Win),
        ("A Z", RPSResult::Loss),
        ("B X", RPSResult::Loss),
        ("B Y", RPSResult::Draw),
        ("B Z", RPSResult::Win),
        ("C X", RPSResult::Win),
        ("C Y", RPSResult::Loss),
        ("C Z", RPSResult::Draw),
    ]);
    let results_table = HashMap::<&str, RPSThrow>::from([
        ("A X", RPSThrow::Scizzors),
        ("A Y", RPSThrow::Rock),
        ("A Z", RPSThrow::Paper),
        ("B X", RPSThrow::Rock),
        ("B Y", RPSThrow::Paper),
        ("B Z", RPSThrow::Scizzors),
        ("C X", RPSThrow::Paper),
        ("C Y", RPSThrow::Scizzors),
        ("C Z", RPSThrow::Rock),
    ]);

    let filename = String::from("input_files/day2");
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut total_points_throw: u64 = 0;
    let mut total_points_result: u64 = 0;
    for matchup in contents.split("\n") {
        if matchup.is_empty() { continue; }
        let mut itr = matchup.split(" ");
        let _them = itr.next().unwrap();
        let encoded = itr.next().unwrap();

        let match_total_if_throw = points_table_throws[encoded] + throw_table[matchup] as u64;
        let match_total_if_result = points_table_results[encoded] + results_table[matchup] as u64;
        total_points_throw += match_total_if_throw;
        total_points_result += match_total_if_result;
    }

    println!("Day 2");
    println!("If throw : {}", total_points_throw);
    println!("If result: {}", total_points_result);
    println!("");
}

fn day_1() {
    let filename = String::from("input_files/day1");
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut heap = BinaryHeap::new();
    for foods in contents.split("\n\n") {
        let mut this_sum = 0;
        for calories in foods.split("\n") {
            this_sum += u32::from_str_radix(calories, 10).unwrap_or(0);
        }
        heap.push(this_sum);
    }

    let top_elf = heap.pop().unwrap();
    let top_three_total = top_elf + heap.pop().unwrap() + heap.pop().unwrap();

    println!("Day 1");
    println!("Top elf     : {} calories total", top_elf);
    println!("Top 3 elves : {} calories total", top_three_total);
    println!("");
}
