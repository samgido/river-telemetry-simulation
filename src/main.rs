use rand::Rng;
use std::{
    collections::HashMap,
    io::{self, Write},
    str::FromStr,
};

struct Fish {
    pos: f32,
    found: bool,
}

fn main() {
    let read_radius: f32;
    let read_time: f32;
    let number_of_fish: i32;
    let float_speed: f32;
    let river_length: f32;

    let test_cases: i32;

    read_radius = read_num("Enter the average read radius of the antenna (m): ");
    read_time = read_num("Enter the read time for each frequency (s): ");
    number_of_fish = read_num("Enter the number of fish: ");
    float_speed = read_num("Enter the speed of the boat (m/s): ");
    river_length = read_num("Enter the length of the river (m): ");

    test_cases = read_num("How many times should the simulation run? ");
    let mut average_found: f32 = 0.0;

    for _ in 0..test_cases {
        let found_count = simulate(
            read_radius,
            read_time,
            number_of_fish,
            float_speed,
            river_length,
        );

        average_found += found_count as f32;
    }

    println!(
        "Found {} of {} fish on average",
        average_found / test_cases as f32,
        number_of_fish
    );
}

fn simulate(
    read_range: f32,
    read_time: f32,
    number_of_fish: i32,
    float_speed: f32,
    river_length: f32,
) -> i32 {
    // random number generator
    let mut rng = rand::thread_rng();

    // maps each fish id to the fishes position and state
    let mut fish_collection: HashMap<i32, Fish> = HashMap::new();

    // how many fish are found
    let mut found_count: i32 = 0;

    let mut current_frequency = 0;
    let mut timer: f32 = 0.0;

    // add a new fish to the hashmap with a random position and unique id
    for i in 0..number_of_fish {
        let random_fish = Fish {
            pos: rng.gen::<f32>() * river_length,
            found: false,
        };

        fish_collection.insert(i, random_fish);
    }

    // simulate readings for each second
    for i in 0..(river_length / float_speed) as i32 {
        // cycling the current frequency
        if timer >= read_time {
            if current_frequency >= number_of_fish - 1 {
                current_frequency = 0;
            } else {
                current_frequency += 1;
            }

            timer = 0.0;
        }

        // calculate the boats position
        let boat_pos = i as f32 * float_speed;

        // checking if the fish that's being listened for is in range
        match fish_collection.get_mut(&current_frequency) {
            Some(fish) => {
                if (fish.pos - boat_pos).abs() <= read_range {
                    fish.found = true;
                }
            }
            None => panic!("A fish escaped!"),
        }

        // updating the timer variable after data is taken
        timer += 1.0;
    }

    // figure how many fish were found
    for (_, v) in fish_collection.iter() {
        if v.found {
            found_count += 1;
        }
    }

    return found_count;
}

// reads a number from the command line
fn read_num<T: FromStr>(message: &str) -> T {
    let mut line = String::new();

    print!("{}", message);
    let _ = io::stdout().flush().expect("stdout flush failed.");

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                if let Ok(value) = line.trim().parse::<T>() {
                    return value;
                } else {
                    println!("Could not parse input, try again");
                }
            }
            Err(_) => {
                println!("Invalid input, try again");
                continue;
            }
        }
    }
}
