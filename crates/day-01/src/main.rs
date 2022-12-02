use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default, Debug, Clone)]
struct ElfCalories(Vec<u64>);

fn main() -> std::io::Result<()> {
    const FILE_NAME: &str = "data/day-01/input";

    let mut elf_calories = ElfCalories::default();

    if let Ok(file) = File::open(FILE_NAME) {
        let reader = BufReader::new(file);

        let mut current_elf: u64 = 0;
        reader.lines().for_each(|line| {
            if let Ok(line) = line {
                if line.is_empty() {
                    elf_calories.0.push(current_elf);
                    current_elf = 0;
                } else {
                    current_elf += line.parse::<u64>().unwrap();
                }
            }
        });
    } else {
        println!("Unable to load file {FILE_NAME}")
    }

    elf_calories
        .0
        .sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    let mut itr = elf_calories.0.iter();

    let top_elf = itr.next().unwrap();
    println!("Top Elf: {:?}", top_elf);

    println!(
        "Top Three Sum: {:?}",
        top_elf + itr.next().unwrap() + itr.next().unwrap()
    );
    Ok(())
}
