extern crate rand;
use std::env;
use rand::Rng;
use rand::seq::SliceRandom;

fn generate_lotto_numbers(num: usize) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut all_numbers: Vec<Vec<u32>> = Vec::new();

    for _n in 0..num {
        let mut red_balls: Vec<u32> = (1..34).collect();
        red_balls.shuffle(&mut rng);
        red_balls.truncate(6);
        red_balls.sort();
        red_balls.push(rng.gen_range(1..=16));  // add blue ball
        all_numbers.push(red_balls);
    }
    
    all_numbers
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let num = if args.len() > 1 {
        args[1].parse().unwrap_or(1)
    } else {
        1
    };

    let all_numbers = generate_lotto_numbers(num);
    println!("双色球号码如下：");
    for (i, numbers) in all_numbers.iter().enumerate() {
        println!("号码{}: {:?}", i + 1, numbers.iter().map(|num| format!("{:02}", num)).collect::<Vec<_>>().join(" "));
    }
}