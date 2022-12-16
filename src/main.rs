use std::collections::{HashMap, BinaryHeap};

fn main() {
    // day_1();
    day_2();
}

#[derive(Copy, Clone)]
enum RPSResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}
fn day_2() {
    let points_table = HashMap::<&str, u64>::from([
        ("A", 1),
        ("B", 2),
        ("C", 3),
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let results_table = HashMap::<&str, RPSResult>::from([
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

    let filename = String::from("input_files/day2");
    let contents = std::fs::read_to_string(filename).unwrap();

    let mut total_points: u64 = 0;
    for matchup in contents.split("\n") {
        if matchup.is_empty() { continue; }
        let mut itr = matchup.split(" ");
        let _them = itr.next().unwrap();
        let me = itr.next().unwrap();

        let match_total = points_table[me] + results_table[matchup] as u64;
        total_points += match_total;
    }

    println!("{}", total_points);
}

fn day_1() {
    let filename = String::from("input_files/day1");
    let contents = std::fs::read_to_string(filename).unwrap();
    let mut heap = BinaryHeap::new();
    for foods in contents.split("\n\n") {
        println!("new elf");
        let mut this_sum = 0;
        for calories in foods.split("\n") {
            println!("this calories is {}", calories);
            this_sum += u32::from_str_radix(calories, 10).unwrap_or(0);
        }
        println!("this elf cals: {}", this_sum);
        heap.push(this_sum);
        println!("");
    }

    let top_three_total = heap.pop().unwrap() + heap.pop().unwrap() + heap.pop().unwrap();

    println!("{}", top_three_total);
}
