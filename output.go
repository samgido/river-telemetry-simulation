package main

import "fmt"

func print_simulation_results(algebraic_estimate float32, average_found float32, time_took float32) {
	fmt.Println("The simulation took", time_took, "seconds")
	fmt.Println("The average number of fish found was", average_found, "with these parameters.")
	fmt.Println()
	fmt.Println("The algebraic estimate was", algebraic_estimate, "fish with these parameters.")
}

func print_options(options SimulationOptions) {
	fmt.Println("Settings used:")

	fmt.Println("Read range:", options.ReadRange, "m")
	fmt.Println("Read time:", options.ReadTime, "sec")
	fmt.Println("Fish count:", options.FishCount, "fish")
	fmt.Println("Float speed:", options.FloatSpeed, "m/s")
	fmt.Println("River length:", options.RiverLength, "m")
	fmt.Println("Times ran:", options.TimesRan, "times")
	fmt.Println()
}
