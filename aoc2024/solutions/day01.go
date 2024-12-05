package solutions

import (
	"sort"
	"strconv"
	"strings"
)

func SolutionDay01part1(input []string) string {
	left, right := make([]int, 0), make([]int, 0)
	for _, line := range input {
		l := strings.Fields(line)
		letfNum, _ := strconv.Atoi(l[0])
		left = append(left, letfNum)
		rightNum, _ := strconv.Atoi(l[1])
		right = append(right, rightNum)
	}
	sort.Ints(left)
	sort.Ints(right)
	ans := 0
	for i := range left {
		if left[i] <= right[i] {
			ans += right[i] - left[i]
		} else {
			ans += left[i] - right[i]
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay01part2(input []string) string {
	left := make([]int, 0)
	similarityMap := make(map[int]int)
	for _, line := range input {
		l := strings.Fields(line)
		letfNum, _ := strconv.Atoi(l[0])
		rightNum, _ := strconv.Atoi(l[1])
		left = append(left, letfNum)
		similarityMap[rightNum] += 1
	}
	similarityScore := 0
	for _, num := range left {
		similarityScore += num * similarityMap[num]
	}
	return strconv.FormatInt(int64(similarityScore), 10)
}
