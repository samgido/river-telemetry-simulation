use rand::Rng;
use std::{num, thread::current};

const read_range: i32 = 75;
const read_time: i32 = 3;
const num_of_fish: i32 = 30;
const float_speed: i32 = 2;
const river_length: f32 = 100000.0;

struct Fish {
    id: i32,
    pos: f32,
    found: bool,
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut fishes: Vec<Fish> = Vec::new();

    for i in 0..num_of_fish {
        let curr_fish = Fish {
            id: i,
            pos: rng.gen::<f32>() * river_length,
            found: false,
        };

        fishes.push(curr_fish);
    }

    let mut float_pos: f32 = 0.0;
    let mut current_frequency = 0;
    let mut timer = 0;

    for i in 0..(river_length as i32 / float_speed) {
        float_pos = (i * float_speed) as f32;

        if timer >= read_time {
            if (current_frequency == num_of_fish - 1) {
                current_frequency = 0;
            } else {
                current_frequency += 1;
            }

            timer = 0;
        }

        for fish in &mut fishes {
            if (fish.pos - float_pos).abs() <= read_range as f32 && fish.id == current_frequency {
                fish.found = true;
            }
        }

        timer += 1;
    }

    for fish in fishes {
        if fish.found {
            println!("Fish {} was found", fish.id);
        }
    }
}
