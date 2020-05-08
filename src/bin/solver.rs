#![allow(unused_variables)]
use rand::rngs::StdRng;
use rand::SeedableRng;

pub fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        4 => {
            let rng = StdRng::seed_from_u64(10_000);
            run(&rng, &args);
        },
        5 => {
            let seed = args[4].parse::<u64>().expect("Invalid random seed");
            let rng = StdRng::seed_from_u64(seed);
            run(&rng, &args);
        },
        _ => println!("Arguments: <Input file> <SampleSize> <Quantile> [<RandomSeed>]")
    }
}

fn run(rng: &StdRng, args: &Vec<String>) {
    let kps = kps_ce::read_kps_instance(&args[1])
                                            .expect("Could not read input file");

    let sample_size = &args[2].parse::<usize>().expect("Sample size error");
    let quantile = &args[3].parse::<f64>().expect("Quantile error");
}