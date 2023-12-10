use core::time;
use rand::Rng;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Instant;
use std::{
    collections::HashMap,
    io::{self, Write},
    os::windows::thread,
    str::FromStr,
};

struct Fish {
    pos: f32,
    found: bool,
}

#[derive(Clone, Copy)]
struct SimulationParameters {
    read_radius: f32,
    read_time: f32,
    number_of_fish: i32,
    float_speed: f32,
    river_length: f32,
}

fn main() {
    let now = Instant::now();

    let read_radius: f32;
    let read_time: f32;
    let number_of_fish: i32;
    let float_speed: f32;
    let river_length: f32;

    let test_cases: i32;

    let params = SimulationParameters {
        read_radius: read_num("Enter the average read radius of the antenna (m): "),
        read_time: read_num("Enter the read time for each frequency (s): "),
        number_of_fish: read_num("Enter the number of fish: "),
        float_speed: read_num("Enter the speed of the boat (m/s): "),
        river_length: read_num("Enter the length of the river (m): "),
    };

    test_cases = read_num("How many times should the simulation run? ");

    let average_found: f32;
    let thread_count: i32 = 3;

    let counter = Arc::new(Mutex::new(0.0));
    let mut handles = vec![];

    for i in 0..thread_count - 1 {
        let partial_counter = Arc::clone(&counter);
        let handle = std::thread::spawn(move || {
            let mut avg = partial_counter.lock().unwrap();

            *avg += run_simulations(test_cases / 3, &params, i);
        });

        handles.push(handle);
    }

    let partial_counter = Arc::clone(&counter);
    let handle = std::thread::spawn(move || {
        let mut avg = partial_counter.lock().unwrap();

        *avg += run_simulations(
            (test_cases / thread_count) + (test_cases % thread_count),
            &params,
            thread_count - 1,
        );
    });

    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    average_found = *counter.lock().unwrap();

    println!(
        "\nOn average, {} of {} fish were found.\n",
        average_found / (thread_count as f32),
        params.number_of_fish
    );

    let time_elapsed = now.elapsed();
    println!("The simulations took {:.3?} seconds", time_elapsed);
}

fn run_simulations(count: i32, params: &SimulationParameters, thread_id: i32) -> f32 {
    let mut average_found: f32 = 0.0;
    for i in 0..count {
        let found_count = simulate(&params);
        // println!(
        //     "Found {} fish on thread #{}, sim #{}",
        //     found_count,
        //     thread_id,
        //     i + 1
        // );

        average_found += found_count as f32;
    }

    return average_found / count as f32;
}

fn simulate(params: &SimulationParameters) -> i32 {
    // random number generator
    let mut rng = rand::thread_rng();

    // maps each fish id to the fishes position and state
    let mut fish_collection: HashMap<i32, Fish> = HashMap::new();

    // how many fish are found
    let mut found_count: i32 = 0;

    let mut current_frequency = 0;
    let mut timer: f32 = 0.0;

    // add a new fish to the hashmap with a random position and unique id
    for i in 0..params.number_of_fish {
        let random_fish = Fish {
            pos: rng.gen::<f32>() * params.river_length,
            found: false,
        };

        fish_collection.insert(i, random_fish);
    }

    // simulate readings for each second
    for i in 0..(params.river_length / params.float_speed) as i32 {
        // cycling the current frequency
        if timer >= params.read_time {
            if current_frequency >= params.number_of_fish - 1 {
                current_frequency = 0;
            } else {
                current_frequency += 1;
            }

            timer = 0.0;
        }

        // calculate the boats position
        let boat_pos = i as f32 * params.float_speed;

        // checking if the fish that's being listened for is in range
        match fish_collection.get_mut(&current_frequency) {
            Some(fish) => {
                if (fish.pos - boat_pos).abs() <= params.read_radius {
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
