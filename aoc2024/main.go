package main

import (
	"aoc2024/solutions"
	"aoc2024/utils"
	"fmt"
	"os"
	"time"
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

	timeStart := time.Now()

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
	case "11":
		fmt.Println("Day 11 Solution:", solutions.SolutionDay11part1(input), solutions.SolutionDay11part2(input))
	case "12":
		fmt.Println("Day 12 Solution:", solutions.SolutionDay12part1(input), solutions.SolutionDay12part2(input))
	case "13":
		fmt.Println("Day 13 Solution:", solutions.SolutionDay13part1(input), solutions.SolutionDay13part2(input))
	case "14":
		fmt.Println("Day 14 Solution:", solutions.SolutionDay14part1(input), solutions.SolutionDay14part2(input))
	case "15":
		fmt.Println("Day 15 Solution:", solutions.SolutionDay15(input))
	case "16":
		fmt.Println("Day 16 Solution:", solutions.SolutionDay16(input))
	case "17":
		fmt.Println("Day 17 Solution:", solutions.SolutionDay17(input))
	case "18":
		fmt.Println("Day 18 Solution:", solutions.SolutionDay18(input))
	case "19":
		fmt.Println("Day 19 Solution:", solutions.SolutionDay19(input))
	case "20":
		fmt.Println("Day 20 Solution:", solutions.SolutionDay20(input))
	case "21":
		fmt.Println("Day 21 Solution:", solutions.SolutionDay21(input))
	case "22":
		fmt.Println("Day 22 Solution:", solutions.SolutionDay22(input))
	case "23":
		fmt.Println("Day 23 Solution:", solutions.SolutionDay23(input))
	case "24":
		fmt.Println("Day 24 Solution:", solutions.SolutionDay24(input))
	case "25":
		fmt.Println("Day 25 Solution:", solutions.SolutionDay25(input))
	default:
		fmt.Printf("Solution for Day %s not implemented.\n", day)
	}
	fmt.Printf("Time: %.2fms\n", float64(time.Since(timeStart).Microseconds())/1000)
}
