package solutions

import (
	"fmt"
	"strconv"
)

func SolutionDay17(input []string) string {

	part1 := d17p1(input)
	part2 := d17p2(input)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d17p1(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}

func d17p2(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}
