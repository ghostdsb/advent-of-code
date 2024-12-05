package solutions

import (
	"strconv"
	"strings"
)

func SolutionDay02part1(input []string) string {
	safeCount := 0
	for _, report := range input {
		levelNums := make([]int, 0)
		for _, level := range strings.Fields(report) {
			levelNum, _ := strconv.Atoi(level)
			levelNums = append(levelNums, levelNum)
		}
		safeFlow := isLevelSafe(levelNums)
		if safeFlow {
			safeCount += 1
		}
	}
	return strconv.FormatInt(int64(safeCount), 10)
}

func SolutionDay02part2(input []string) string {
	safeCount := 0
	for _, report := range input {
		levelNums := make([]int, 0)
		for _, level := range strings.Fields(report) {
			levelNum, _ := strconv.Atoi(level)
			levelNums = append(levelNums, levelNum)
		}
		safeFlow := isLevelSafe(levelNums)
		if safeFlow {
			safeCount += 1
		} else {
			for i := 0; i < len(levelNums); i++ {
				a := append([]int{}, levelNums[:i]...)
				a = append(a, levelNums[i+1:]...)
				safe := isLevelSafe(a)
				if safe {
					safeCount += 1
					break
				}
			}
		}
	}
	return strconv.FormatInt(int64(safeCount), 10)
}

func isLevelSafe(level []int) bool {
	flowCount := make(map[string]int)
	var last int
	safeFlow := true
	for i, levelNum := range level {
		if i == 0 {
			last = levelNum
		} else {
			if levelNum-last > 0 {
				flowCount["increasing"] += 1
				if levelNum-last > 3 {
					flowCount["big-jump-ahead"] += 1
				}
			} else if levelNum-last < 0 {
				flowCount["decreasing"] += 1
				if levelNum-last < -3 {
					flowCount["big-jump-back"] += 1
				}
			} else {
				flowCount["same"] += 1
			}
			last = levelNum
		}
	}
	if flowCount["big-jump-ahead"] > 0 || flowCount["big-jump-back"] > 0 {
		safeFlow = false
	}
	if flowCount["same"] > 0 {
		safeFlow = false
	}
	if flowCount["increasing"] != 0 && flowCount["decreasing"] != 0 {
		safeFlow = false
	}
	return safeFlow
}
