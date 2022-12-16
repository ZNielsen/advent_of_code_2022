use std::collections::BinaryHeap;

fn main() {
    day_1();
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
