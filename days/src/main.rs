fn main() {
    println!("Hello, world!");
    days::DAYS.iter().enumerate().for_each(|(i, day)| {
        println!("Day {:02}:", i + 1);
        println!("Part 1: {}", day.part1());
        println!("Part 2: {}", day.part2());
    });
}
