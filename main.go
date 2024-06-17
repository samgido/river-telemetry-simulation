package main

import (
	"encoding/json"
	"fmt"
	"math"
	"math/rand"
	"os"
)

type SimulationOptions struct {
	ReadRange   float32
	ReadTime    float32
	FishCount   int
	FloatSpeed  float32
	RiverLength float32
	TimesRan    int
}

type Fish struct {
	position float32
	is_found bool
}

func check(e error) {
	if e != nil {
		panic(e)
	}
}

func read_options() SimulationOptions {
	file, err := os.ReadFile("options.json")
	check(err)

	var options SimulationOptions
	err = json.Unmarshal(file, &options)
	check(err)

	fmt.Println(options)

	return options
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

			fmt.Println("Found fish ", distance, " meters away")
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

func main() {
	var options SimulationOptions
	options = read_options()
	fmt.Println(average_simulations(options))
}
