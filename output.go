package main

import (
	"fmt"
	"os"
	"strings"
	"time"
)

func output_results(options SimulationOptions, algebraic_estimate float32, average_found float32, time_took float32) {
	var sb strings.Builder

	sb.WriteString(output_options(options))

	sb.WriteString("The simulation took ")
	sb.WriteString(fmt.Sprintf("%.2f", time_took))
	sb.WriteString(" seconds\n")
	sb.WriteString("The average number of fish found was ")
	sb.WriteString(fmt.Sprintf("%.2f", average_found))
	sb.WriteString("\n\n")
	sb.WriteString("The algebraic estimate was ")
	sb.WriteString(fmt.Sprintf("%.2f", algebraic_estimate))
	sb.WriteString("\n")

	output := sb.String()
	fmt.Print(output)

	currentTime := time.Now()
	fileName := currentTime.Format("2006-10-2 15-04-05") + ".txt"

	file, err := os.Create("./logs/" + fileName)
	check(err)
	defer file.Close()

	_, err = file.WriteString(output)
	check(err)

	fmt.Println("\nOutput written to file:", fileName)
}

func output_options(options SimulationOptions) string {
	var sb strings.Builder
	sb.WriteString("Simulation parameters:\n")

	sb.WriteString("Read range: ")
	sb.WriteString(fmt.Sprintf("%.1f", options.ReadRange))
	sb.WriteString(" m\n")
	sb.WriteString("Read time: ")
	sb.WriteString(fmt.Sprintf("%.1f", options.ReadTime))
	sb.WriteString(" sec\n")
	sb.WriteString("Fish count: ")
	sb.WriteString(fmt.Sprintf("%d", options.FishCount))
	sb.WriteString(" fish\n")
	sb.WriteString("Float speed: ")
	sb.WriteString(fmt.Sprintf("%.1f", options.FloatSpeed))
	sb.WriteString(" sec\n")
	sb.WriteString("River length: ")
	sb.WriteString(fmt.Sprintf("%.1f", options.RiverLength))
	sb.WriteString(" m\n")
	sb.WriteString("Times ran: ")
	sb.WriteString(fmt.Sprintf("%d", options.TimesRan))
	sb.WriteString(" sec\n\n")

	return sb.String()
}
