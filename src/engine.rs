use rand::Rng;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub struct Engine {
    pub read_radius: f32,
    pub read_time: f32,
    pub number_of_fish: i32,
    pub float_speed: f32,
    pub river_length: f32,
    pub times_ran: i32,
}

impl Engine {
    pub fn new(
        read_radius: f32,
        read_time: f32,
        number_of_fish: i32,
        float_speed: f32,
        river_length: f32,
    ) -> Engine {
        Engine {
            read_radius,
            read_time,
            number_of_fish,
            float_speed,
            river_length,
            times_ran: 0,
        }
    }

    pub fn calculate_algebraic_estimate(&self) -> f32 {
        (2.0 * self.read_radius) / (self.float_speed * self.read_time)
    }

    pub fn simulate(&mut self) -> i32 {
        let mut rng = rand::thread_rng();

        let mut fish_collection: HashMap<i32, Fish> = HashMap::new();
        let mut found_count: i32 = 0;

        for i in 0..self.number_of_fish {
            let random_fish = Fish {
                pos: rng.gen::<f32>() * self.river_length,
                found: false,
            };

            fish_collection.insert(i, random_fish);
        }

        let mut current_frequency = 0;
        let mut timer = 0.0;

        for i in 0..(self.river_length / self.float_speed) as i32 {
            // cycling the current frequency
            if timer >= self.read_time {
                if current_frequency >= self.number_of_fish - 1 {
                    current_frequency = 0;
                } else {
                    current_frequency += 1;
                }

                timer = 0.0;
            }

            // updating the timer variable after data is taken
            timer += 1.0;

            // calculate the boats position
            let boat_pos = i as f32 * self.float_speed;

            // checking if the fish that's being listened for is in range
            match fish_collection.get_mut(&current_frequency) {
                Some(fish) => {
                    if (fish.pos - boat_pos).abs() <= self.read_radius {
                        fish.found = true;
                    }
                }
                None => panic!("A fish escaped!"),
            }
        }

        for f in fish_collection.values() {
            if f.found {
                found_count += 1;
            }
        }

        self.times_ran += 1;
        return found_count;
    }
}

struct Fish {
    pos: f32,
    found: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn engine_constructor_test_ok() {
        let engine = Engine::new(75.0, 3.0, 30, 1.3, 100000.0);
        assert_eq!(
            engine,
            Engine {
                read_radius: 75.0,
                read_time: 3.0,
                number_of_fish: 30,
                float_speed: 1.3,
                river_length: 100000.0,
                times_ran: 0
            }
        );
    }

    #[test]
    fn engine_constructor_test_fail() {
        let engine = Engine::new(75.0, 3.0, 30, 1.3, 100000.0);
        assert_ne!(
            engine,
            Engine {
                read_radius: 100.0,
                read_time: 3.0,
                number_of_fish: 30,
                float_speed: 1.3,
                river_length: 100000.0,
                times_ran: 0,
            }
        );
    }

    #[test]
    fn algebraic_estimate_test_ok() {
        let engine = Engine::new(75.0, 3.0, 30, 1.3, 100000.0);
        let result = engine.calculate_algebraic_estimate();

        assert_eq!(result, 38.46154);
    }

    #[test]
    fn algebraic_estimate_test_fail() {
        let engine = Engine::new(75.0, 3.0, 30, 1.3, 100000.0);
        let result = engine.calculate_algebraic_estimate();

        assert_ne!(result, 0.0);
    }

    #[test]
    fn simulate_test_ok() {
        let mut engine = Engine::new(75.0, 3.0, 30, 1.3, 100000.0);
        let result = engine.simulate();

        assert_eq!(result, 30);
    }
}
