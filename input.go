package main

import (
	"fmt"
	"os"

	"github.com/BurntSushi/toml"
)

type SimulationOptions struct {
	ReadRange   float32
	ReadTime    float32
	FishCount   int
	FloatSpeed  float32
	RiverLength float32
	TimesRan    int
}

func read_options() SimulationOptions {
	file, err := os.ReadFile("options.toml")
	check(err)

	var options SimulationOptions
	err = toml.Unmarshal(file, &options)
	check(err)

	print_options(options)

	return options
}

func print_simulation_results(algebraic_estimate float32, average_found float32, time_took float32) {
	fmt.Println("The simulation took", time_took, "seconds")
	fmt.Println("The average number of fish found was", average_found, "with these parameters.")
	fmt.Println()
	fmt.Println("The algebraic estimate was", algebraic_estimate, "fish with these parameters.")
}

func print_options(options SimulationOptions) {
	fmt.Println("Settings used:")

	fmt.Println("ReadRange:", options.ReadRange)
	fmt.Println("ReadTime:", options.ReadTime)
	fmt.Println("FishCount:", options.FishCount)
	fmt.Println("FloatSpeed:", options.FloatSpeed)
	fmt.Println("RiverLength:", options.RiverLength)
	fmt.Println("TimesRan:", options.TimesRan)
	fmt.Println()
}
