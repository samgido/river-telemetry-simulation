use chrono;
use rand::Rng;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufWriter, Write},
    str::FromStr,
    time::Instant,
};

use crate::engine::Engine;
pub mod engine;

struct Fish {
    pos: f32,
    found: bool,
}

fn main() {
    let mut read_radius: f32;
    let mut read_time: f32;
    let mut number_of_fish: i32;
    let mut float_speed: f32;
    let mut river_length: f32;

    let mut test_cases: i32;

    let mut repeat = true;
    while repeat {
        read_radius = read_num("Enter the average read radius of the antenna (m): ");
        read_time = read_num("Enter the read time for each frequency (s): ");
        number_of_fish = read_num("Enter the number of fish: ");
        float_speed = read_num("Enter the speed of the boat (m/s): ");
        river_length = read_num("Enter the length of the river (m): ");

        test_cases = read_num("How many times should the simulation run? ");
        let mut average_found: f32 = 0.0;
        let engine = Engine::new(
            read_radius,
            read_time,
            number_of_fish,
            float_speed,
            river_length,
        );

        let now = Instant::now();
        for i in 0..test_cases {
            let found_count = engine.simulate();

            println!("Running simulation #{}", i + 1);
            println!("Simulation #{} found {} fish", i + 1, found_count);

            average_found += found_count as f32;
        }
        let time_elapsed = now.elapsed();
        println!("The simulations finished in {:.3?} seconds", time_elapsed);

        let average_found = average_found / test_cases as f32;
        println!(
            "\nOn average, {} of {} fish were found with these parameters.",
            average_found, number_of_fish
        );

        let algebraic_estimate = engine.calculate_algebraic_estimate();

        if algebraic_estimate > number_of_fish as f32 {
            println!("The algebraic estimate was {} fish.", number_of_fish);
        } else {
            println!("The algebraic estimate was {} fish.", algebraic_estimate);
        }

        print(engine, average_found);

        print!("\nRun the program again? (y/n): ");
        let _ = io::stdout().flush().expect("stdout flush failed.");
        while repeat {
            let mut repeat_string = String::new();
            match io::stdin().read_line(&mut repeat_string) {
                Ok(_) => {
                    if repeat_string.trim() == "Y" || repeat_string.trim() == "y" {
                        repeat = true;
                        println!("\n");
                        break;
                    } else if repeat_string.trim() == "N" || repeat_string.trim() == "n" {
                        repeat = false;
                        break;
                    } else {
                        print!("Choose one of the options, (y/n): ");
                        let _ = io::stdout().flush().expect("stdout flush failed.");
                    }
                }
                Err(_) => {
                    print!("Could not read input, try again:");
                    let _ = io::stdout().flush().expect("stdout flush failed.");
                    continue;
                }
            }
        }

        if !repeat {
            println!("\nSource code at github.com/samgido/river-telemetry-simulation");
        }
    }
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
                    print!("Could not parse input, try again: ");
                    let _ = io::stdout().flush().expect("stdout flush failed.");
                    line = String::new();
                }
            }
            Err(_) => {
                print!("Invalid input, try again:");
                let _ = io::stdout().flush().expect("stdout flush failed.");
                line = String::new();
                continue;
            }
        }
    }
}

fn print(engine: Engine, found: f32) {
    let mut filename = String::new();
    let now = chrono::offset::Local::now();
    filename += "logs/";
    filename += &format!("{}", now.format("%I%M%S-%d%m%Y"));
    filename += ".txt";

    let file: File;

    if let Ok(f) = File::create(filename) {
        file = f;
    } else {
        println!("Couldn't create log.");
        return;
    }

    let mut content: String = String::new();
    content += "Read range: ";
    content += engine.read_radius.to_string().as_str();
    content += "\n";
    content += "Read time: ";
    content += engine.read_time.to_string().as_str();
    content += "\n";
    content += "Number of fish: ";
    content += engine.number_of_fish.to_string().as_str();
    content += "\n";
    content += "Float speed: ";
    content += engine.float_speed.to_string().as_str();
    content += "\n";
    content += "River length: ";
    content += engine.river_length.to_string().as_str();
    content += "\n";
    content += "# of fish found by simulations: ";
    content += found.to_string().as_str();
    content += "\n";
    content += "Algebraic estimate: ";
    content += engine.calculate_algebraic_estimate().to_string().as_str();

    let mut f = BufWriter::new(file);
    f.write_all(content.as_bytes())
        .expect("Unable to write to file");
}

// The Celestial Suite; Pale Jay
// Patience; Tame Impala
// 33; Insightful
