package solutions

import (
	"aoc2024/utils"
	"strconv"
)

func SolutionDay08part1(input []string) string {
	ans := 0
	grid := make([][]rune, 0)
	antinodeGrid := make([][]rune, 0)
	frequencyLocation := make(map[rune][][]int)
	for r, line := range input {
		row := make([]rune, 0)
		antinodeR := make([]rune, 0)
		for c, pos := range line {
			row = append(row, pos)
			antinodeR = append(antinodeR, '.')
			if pos != '.' {
				if _, ok := frequencyLocation[pos]; ok {
					frequencyLocation[pos] = append(frequencyLocation[pos], []int{r, c})
				} else {
					frequencyLocation[pos] = [][]int{{r, c}}
				}
			}
		}
		grid = append(grid, row)
		antinodeGrid = append(antinodeGrid, antinodeR)
	}
	for _, nodes := range frequencyLocation {
		for i := 0; i < len(nodes)-1; i++ {
			for j := i + 1; j < len(nodes); j++ {
				antinodeA, antinodeB := findAntinode(nodes[i], nodes[j])
				if antinodeA[0] >= 0 && antinodeA[1] >= 0 &&
					antinodeA[0] < len(grid) && antinodeA[1] < len(grid[0]) {
					antinodeGrid[antinodeA[0]][antinodeA[1]] = '#'
				}
				if antinodeB[0] >= 0 && antinodeB[1] >= 0 &&
					antinodeB[0] < len(grid) && antinodeB[1] < len(grid[0]) {
					antinodeGrid[antinodeB[0]][antinodeB[1]] = '#'
				}
			}
		}
	}
	utils.PrintGrid(antinodeGrid)
	for _, row := range antinodeGrid {
		for _, freq := range row {
			if freq == '#' {
				ans += 1
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func findAntinode(nodeA []int, nodeB []int) ([]int, []int) {
	x1, y1 := nodeA[0], nodeA[1]
	x2, y2 := nodeB[0], nodeB[1]
	dx, dy := x1-x2, y1-y2
	x3, y3 := x1+dx, y1+dy
	x4, y4 := x2-dx, y2-dy
	return []int{x3, y3}, []int{x4, y4}
}

func findAntinode2(nodeA []int, nodeB []int, lineLength int) [][]int {
	x1, y1 := nodeA[0], nodeA[1]
	x2, y2 := nodeB[0], nodeB[1]
	dx, dy := x1-x2, y1-y2
	lines := make([][]int, 0)
	lines = append(lines, []int{x1, y1})
	lines = append(lines, []int{x2, y2})
	for i := range lineLength {
		lines = append(lines, []int{x1 + dx*(i+1), y1 + dy*(i+1)})
	}
	for i := range lineLength {
		lines = append(lines, []int{x2 - dx*(i+1), y2 - dy*(i+1)})
	}
	return lines
}

func SolutionDay08part2(input []string) string {
	ans := 0
	grid := make([][]rune, 0)
	antinodeGrid := make([][]rune, 0)
	frequencyLocation := make(map[rune][][]int)
	for r, line := range input {
		row := make([]rune, 0)
		antinodeR := make([]rune, 0)
		for c, pos := range line {
			row = append(row, pos)
			antinodeR = append(antinodeR, '.')
			if pos != '.' {
				if _, ok := frequencyLocation[pos]; ok {
					frequencyLocation[pos] = append(frequencyLocation[pos], []int{r, c})
				} else {
					frequencyLocation[pos] = [][]int{{r, c}}
				}
			}
		}
		grid = append(grid, row)
		antinodeGrid = append(antinodeGrid, antinodeR)
	}
	for _, nodes := range frequencyLocation {
		for i := 0; i < len(nodes)-1; i++ {
			for j := i + 1; j < len(nodes); j++ {
				antinodeList := findAntinode2(nodes[i], nodes[j], max(len(grid), len(grid[0])))
				for _, antiNode := range antinodeList {
					if antiNode[0] >= 0 && antiNode[1] >= 0 &&
						antiNode[0] < len(grid) && antiNode[1] < len(grid[0]) {
						antinodeGrid[antiNode[0]][antiNode[1]] = '#'
					}
				}
			}
		}
	}
	utils.PrintGrid(antinodeGrid)
	for _, row := range antinodeGrid {
		for _, freq := range row {
			if freq == '#' {
				ans += 1
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func max(a, b int) int {
	if a > b {
		return a
	}
	return b
}
