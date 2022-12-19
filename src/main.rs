use std::collections::{HashMap, HashSet, BinaryHeap};

fn main() {
    day_1();
    day_2();
    day_3();
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

    println!("Day 2:");
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

    let top_three_total = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();

    println!("Day 1:");
    println!("Top 3 elves have {} calories total", top_three_total);
    println!("");
}
