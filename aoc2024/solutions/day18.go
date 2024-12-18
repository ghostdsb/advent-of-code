package solutions

import (
	"fmt"
	"strconv"
)

func SolutionDay18(input []string) string {

	part1 := d18p1(input)
	part2 := d18p2(input)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d18p1(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}

func d18p2(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}
