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
	case "6":
		fmt.Println("Day 6 Solution:", solutions.SolutionDay06part1(input), solutions.SolutionDay06part2(input))
	case "7":
		fmt.Println("Day 7 Solution:", solutions.SolutionDay07part1(input), solutions.SolutionDay07part2(input))
	case "8":
		fmt.Println("Day 8 Solution:", solutions.SolutionDay08part1(input), solutions.SolutionDay08part2(input))
	case "9":
		fmt.Println("Day 9 Solution:", solutions.SolutionDay09part1(input), solutions.SolutionDay09part2(input))
	case "10":
		fmt.Println("Day 10 Solution:", solutions.SolutionDay10part1(input), solutions.SolutionDay10part2(input))
	default:
		fmt.Printf("Solution for Day %s not implemented.\n", day)
	}
}
