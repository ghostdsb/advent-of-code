package solutions

import (
	"fmt"
	"strconv"
)

func SolutionDay10part1(input []string) string {
	ans := 0
	heightMapGrid := make([][]int, 0)
	flow := make(map[string]map[string]struct{})
	for _, line := range input {
		row := make([]int, 0)
		for _, char := range line {
			height, err := strconv.Atoi(string(char))
			if err != nil {
				row = append(row, -1)
			} else {
				row = append(row, height)
			}
		}
		heightMapGrid = append(heightMapGrid, row)
	}
	stack := make([][]int, 0)
	for r, line := range heightMapGrid {
		for c, height := range line {
			if height == 0 {
				stack = append(stack, []int{r, c})
			}
		}
	}
	dir := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	start := ""
	for len(stack) > 0 {
		last := stack[len(stack)-1]
		if heightMapGrid[last[0]][last[1]] == 0 {
			start = fmt.Sprintf("%d:%d", last[0], last[1])
			flow[start] = make(map[string]struct{})
		}
		if heightMapGrid[last[0]][last[1]] == 9 {
			end := fmt.Sprintf("%d:%d", last[0], last[1])
			if _, ok := flow[start][end]; !ok {
				flow[start][end] = struct{}{}
				ans += 1
			}
		}
		stack = stack[:len(stack)-1]
		for _, d := range dir {
			nx := last[0] + d[0]
			ny := last[1] + d[1]
			if nx >= 0 && ny >= 0 && nx < len(heightMapGrid) && ny < len(heightMapGrid[0]) {
				if heightMapGrid[nx][ny] == heightMapGrid[last[0]][last[1]]+1 {
					stack = append(stack, []int{nx, ny})
				}
			}
		}

	}
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay10part2(input []string) string {
	ans := 0
	heightMapGrid := make([][]int, 0)
	flow := make(map[string]map[string]struct{})
	for _, line := range input {
		row := make([]int, 0)
		for _, char := range line {
			height, err := strconv.Atoi(string(char))
			if err != nil {
				row = append(row, -1)
			} else {
				row = append(row, height)
			}
		}
		heightMapGrid = append(heightMapGrid, row)
	}
	stack := make([][]int, 0)
	for r, line := range heightMapGrid {
		for c, height := range line {
			if height == 0 {
				stack = append(stack, []int{r, c})
			}
		}
	}
	dir := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	start := ""
	for len(stack) > 0 {
		last := stack[len(stack)-1]
		if heightMapGrid[last[0]][last[1]] == 0 {
			start = fmt.Sprintf("%d:%d", last[0], last[1])
			flow[start] = make(map[string]struct{})
		}
		if heightMapGrid[last[0]][last[1]] == 9 {
			ans += 1
			end := fmt.Sprintf("%d:%d", last[0], last[1])
			if _, ok := flow[start][end]; !ok {
				flow[start][end] = struct{}{}
				// ans += 1
			}
		}
		stack = stack[:len(stack)-1]
		for _, d := range dir {
			nx := last[0] + d[0]
			ny := last[1] + d[1]
			if nx >= 0 && ny >= 0 && nx < len(heightMapGrid) && ny < len(heightMapGrid[0]) {
				if heightMapGrid[nx][ny] == heightMapGrid[last[0]][last[1]]+1 {
					stack = append(stack, []int{nx, ny})
				}
			}
		}

	}
	return strconv.FormatInt(int64(ans), 10)
}
