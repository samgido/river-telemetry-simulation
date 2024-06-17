package main

import (
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
