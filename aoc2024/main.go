package main

import (
	"aoc2024/solutions"
	"aoc2024/utils"
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: go run main.go <day>")
		return
	}

	day := os.Args[1]
	var inputFolder string
	if len(os.Args) == 3 {
		inputFolder = "sample_inputs"
	} else {
		inputFolder = "inputs"
	}
	inputFile := fmt.Sprintf("%s/day%s.txt", inputFolder, day)

	input, err := utils.ReadFile(inputFile)
	if err != nil {
		fmt.Printf("Error reading input for Day %s: %v\n", day, err)
		return
	}

	switch day {
	case "1":
		fmt.Println("Day 1 Solution:", solutions.SolutionDay01part1(input), solutions.SolutionDay01part2(input))
	case "2":
		fmt.Println("Day 2 Solution:", solutions.SolutionDay02part1(input), solutions.SolutionDay02part2(input))
	case "3":
		fmt.Println("Day 3 Solution:", solutions.SolutionDay03part1(input), solutions.SolutionDay03part2(input))
	case "4":
		fmt.Println("Day 4 Solution:", solutions.SolutionDay04part1(input), solutions.SolutionDay04part2(input))
	case "5":
		fmt.Println("Day 5 Solution:", solutions.SolutionDay05part1(input), solutions.SolutionDay05part2(input))
	default:
		fmt.Printf("Solution for Day %s not implemented.\n", day)
	}
}
