mod knapsack;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


/// Format:
///    capacity
///    item_1 value_1 weight_1 setup_costs_1 setup_time_1
///    item_2 value_2 weight_2 setup_costs_2 setup_time_2
///    ...
///    item_n value_n weight_n setup_costs_n setup_time_n
pub fn read_kps_instance(input_file: &str) -> Option<knapsack::KnapsackWithSetups> {
    if let Ok(mut lines) = read_lines(input_file) {
        let capacity = lines.next().unwrap().unwrap().parse::<usize>().unwrap();

        let mut items: Vec<knapsack::ItemWithSetup> = Vec::new();
        for line in lines {
            let data = line.unwrap();
            items.push(knapsack::ItemWithSetup::from_string(data));
        }
        Some(knapsack::KnapsackWithSetups::new(items, capacity))
    } else {
        None
    }
}

fn read_lines<P>(file_name: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path> {
    let file: File = File::open(file_name)?;
    Ok(io::BufReader::new(file).lines())
}