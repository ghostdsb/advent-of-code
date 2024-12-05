package solutions

import (
	"regexp"
	"strconv"
)

func SolutionDay03part1(input []string) string {
	ans := 0
	for _, line := range input {
		pattern := `mul\(([0-9]{1,3}),\s*([0-9]{1,3})\)`
		re := regexp.MustCompile(pattern)
		matches := re.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			a, _ := strconv.Atoi(match[1])
			b, _ := strconv.Atoi(match[2])
			mul := a * b
			ans += mul
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay03part2(input []string) string {
	ans := 0
	mulEnabled := true
	for _, line := range input {
		pattern := `mul\(([0-9]{1,3}),\s*([0-9]{1,3})\)|do\(\)|don't\(\)`
		re := regexp.MustCompile(pattern)
		matches := re.FindAllStringSubmatch(line, -1)
		for _, match := range matches {
			if match[0] == "do()" {
				mulEnabled = true
			} else if match[0] == "don't()" {
				mulEnabled = false
			}
			if mulEnabled {
				a, _ := strconv.Atoi(match[1])
				b, _ := strconv.Atoi(match[2])
				mul := a * b
				ans += mul
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}
