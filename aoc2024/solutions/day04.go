package solutions

import (
	"strconv"
)

/*
search for 'X'
directions to move : {1,0}, {1,1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1}
*/

func SolutionDay04part1(input []string) string {
	ans := 0
	grid := make([][]rune, 0)
	for _, line := range input {
		row := make([]rune, 0)
		for _, char := range line {
			row = append(row, char)
		}
		grid = append(grid, row)
	}
	ans = findXmas(grid)
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay04part2(input []string) string {
	ans := 0
	grid := make([][]rune, 0)
	for _, line := range input {
		row := make([]rune, 0)
		for _, char := range line {
			row = append(row, char)
		}
		grid = append(grid, row)
	}
	ans = findCrossMas(grid)
	return strconv.FormatInt(int64(ans), 10)
}

func findXmas(grid [][]rune) int {
	word := "XMAS"
	directions := [][]int{
		{1, 0}, {1, 1}, {0, 1}, {-1, 1}, {-1, 0}, {-1, -1}, {0, -1}, {1, -1},
	}
	xmasCount := 0
	for i := 0; i < len(grid); i++ {
		for j := 0; j < len(grid[0]); j++ {
			for _, dir := range directions {
				found := true
				for k := 0; k < len(word); k++ {
					nx, ny := i+dir[0]*k, j+dir[1]*k
					if nx < 0 || nx >= len(grid) ||
						ny < 0 || ny >= len(grid[0]) ||
						string(grid[nx][ny]) != string(word[k]) {
						found = false
					}
				}
				if found {
					xmasCount += 1
				}
			}

		}
	}
	return xmasCount
}

func findCrossMas(grid [][]rune) int {
	xmasCount := 0
	for i := 0; i <= len(grid)-3; i++ {
		for j := 0; j <= len(grid[i])-3; j++ {
			found := false
			ninexnine := [][]rune{}
			for x := 0; x < 3; x++ {
				l := make([]rune, 0)
				for y := 0; y < 3; y++ {
					l = append(l, grid[i+x][j+y])
				}
				ninexnine = append(ninexnine, l)
			}
			if ninexnine[1][1] == 'A' && ninexnine[0][0]+ninexnine[2][2] == 'M'+'S' &&
				ninexnine[0][2]+ninexnine[2][0] == 'M'+'S' {
				found = true
			}
			if found {
				xmasCount += 1
			}
		}
	}
	return xmasCount
}
