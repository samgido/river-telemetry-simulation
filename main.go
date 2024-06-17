package main

import (
	"math"
	"math/rand"
	"time"
)

type Fish struct {
	position float32
	is_found bool
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func simulate(options SimulationOptions) int {
	fishes := make(map[int]Fish)

	// generate fish
	for i := 0; i < options.FishCount; i++ {
		position := rand.Float32() * options.RiverLength
		fishes[i] = Fish{position: position, is_found: false}
	}

	var current_frequency = 0
	var timer = 0

	for i := 0; i < int(options.RiverLength/options.FloatSpeed); i++ {
		if float32(timer) >= options.ReadTime {
			if current_frequency >= options.FishCount {
				current_frequency = 0
			} else {
				current_frequency += 1
			}

			timer = 0
		}

		timer += 1

		var boat_pos = float32(i) * options.FloatSpeed

		var fish, present = fishes[i]
		if !present {
			continue
		}

		var distance = fish.position - boat_pos
		if math.Abs(float64(distance)) <= float64(options.ReadRange) {
			fish.is_found = true
			fishes[i] = fish // Update the value of the fish in the map
		}
	}

	var found_count = 0
	for _, fish := range fishes {
		if fish.is_found {
			found_count += 1
		}
	}

	return found_count
}

func average_simulations(options SimulationOptions) float32 {
	var average_found float32 = 0

	for i := 0; i < options.TimesRan; i++ {
		average_found += float32(simulate(options))
	}

	return average_found / float32(options.TimesRan)
}

func algebraic_estimate(options SimulationOptions) float32 {
	return (2 * options.ReadRange) / (options.FloatSpeed * options.ReadTime)
}

func main() {
	var options SimulationOptions
	options = read_options()

	start := time.Now()
	simulation_results := average_simulations(options)
	duration := time.Since(start)

	algebraic_estimate := algebraic_estimate(options)

	print_simulation_results(algebraic_estimate, simulation_results, float32(duration))
}
