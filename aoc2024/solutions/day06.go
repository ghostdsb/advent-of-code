package solutions

import (
	"aoc2024/utils"
	"fmt"
	"strconv"
)

func SolutionDay06part1(input []string) string {
	ans := 0
	startR, startC := 0, 0
	dir := []int{0, 0}
	grid := make([][]rune, 0)
	for r, line := range input {
		row := make([]rune, 0)
		for c, pos := range line {
			row = append(row, pos)
			if pos == '^' || pos == '>' ||
				pos == 'v' || pos == '<' {
				startR, startC = r, c
			}
			if pos == '^' {
				dir = []int{-1, 0}
			} else if pos == '>' {
				dir = []int{0, 1}
			} else if pos == 'v' {
				dir = []int{1, 0}
			} else if pos == '<' {
				dir = []int{0, -1}
			}
		}
		grid = append(grid, row)
	}
	march(grid, startR, startC, dir)
	utils.PrintGrid(grid)
	for _, line := range grid {
		for _, sq := range line {
			if sq == 'O' {
				ans += 1
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

type spot struct {
	pos rune
	dir []int
}

func SolutionDay06part2(input []string) string {
	startR, startC := 0, 0
	dir := []int{0, 0}
	grid := make([][]spot, 0)
	originalGrid := make([][]rune, 0)
	for r, line := range input {
		row := make([]spot, 0)
		originalRow := make([]rune, 0)
		for c, pos := range line {
			spt := spot{pos: pos, dir: []int{0, 0}}
			isPlayer := false
			if pos == '^' || pos == '>' ||
				pos == 'v' || pos == '<' {
				startR, startC = r, c
				isPlayer = true
			}
			if pos == '^' {
				dir = []int{-1, 0}
			} else if pos == '>' {
				dir = []int{0, 1}
			} else if pos == 'v' {
				dir = []int{1, 0}
			} else if pos == '<' {
				dir = []int{0, -1}
			}
			if isPlayer {
				spt.dir = dir
			}
			row = append(row, spt)
			originalRow = append(originalRow, pos)
		}
		originalGrid = append(originalGrid, originalRow)
		grid = append(grid, row)
	}
	march(originalGrid, startR, startC, dir)
	obstacles := make([][]int, 0)
	for r, row := range originalGrid {
		for c, col := range row {
			if col == 'O' {
				obstacles = append(obstacles, []int{r, c})
			}
		}
	}

	crosses := 0
	gridCopy := make([][]spot, len(grid))
	for i := range grid {
		gridCopy[i] = make([]spot, len(grid[i]))
		copy(gridCopy[i], grid[i])
	}

	for _, obstacle := range obstacles {
		gridCopy[obstacle[0]][obstacle[1]].pos = '#'
		if isLooped(gridCopy, startR, startC, dir, true) {
			crosses += 1
		}
		gridCopy = make([][]spot, len(grid))
		for i := range grid {
			gridCopy[i] = make([]spot, len(grid[i]))
			copy(gridCopy[i], grid[i])
		}
	}
	fmt.Println("CROSSES", crosses)

	return strconv.FormatInt(int64(crosses), 10)
}

func march(grid [][]rune, r, c int, dir []int) {
	if r < 0 || c < 0 {
		return
	}
	if r == len(grid) || c == len(grid[0]) {
		return
	}
	grid[r][c] = 'O'
	r = r + dir[0]
	c = c + dir[1]
	peekNextPosR := r + dir[0]
	peekNextPosC := c + dir[1]
	if !(peekNextPosR < 0 || peekNextPosC < 0 || peekNextPosR >= len(grid) || peekNextPosC >= len(grid[0])) {
		if grid[peekNextPosR][peekNextPosC] == '#' {
			dir = nextDir(dir)
		}
	}
	march(grid, r, c, dir)
}

func isLooped(grid [][]spot, r, c int, dir []int, isStart bool) bool {
	if r < 0 || c < 0 {
		return false
	}
	if r == len(grid) || c == len(grid[0]) {
		return false
	}
	var dirChar rune

	if dir[0] == 0 && dir[1] == 1 {
		dirChar = '>'
	} else if dir[0] == 0 && dir[1] == -1 {
		dirChar = '<'
	} else if dir[0] == 1 && dir[1] == 0 {
		dirChar = 'v'
	} else {
		dirChar = '^'
	}

	if !isStart {
		if grid[r][c].dir[0] == 0 && grid[r][c].dir[1] == -1 && dirChar == '<' {
			return true
		} else if grid[r][c].dir[0] == 1 && grid[r][c].dir[1] == 0 && dirChar == 'v' {
			return true
		} else if grid[r][c].dir[0] == 0 && grid[r][c].dir[1] == 1 && dirChar == '>' {
			return true
		} else if grid[r][c].dir[0] == -1 && grid[r][c].dir[1] == 0 && dirChar == '^' {
			return true
		}
	}

	grid[r][c] = spot{pos: dirChar, dir: dir}
	r = r + dir[0]
	c = c + dir[1]
	peekNextPosR := r + dir[0]
	peekNextPosC := c + dir[1]
	if !(peekNextPosR < 0 || peekNextPosC < 0 || peekNextPosR >= len(grid) || peekNextPosC >= len(grid[0])) {
		if grid[peekNextPosR][peekNextPosC].pos == '#' {
			dir = nextDir(dir)
		}
	}
	return isLooped(grid, r, c, dir, false)
}

func nextDir(currentDir []int) []int {
	r, c := currentDir[0], currentDir[1]
	if r == -1 && c == 0 {
		return []int{0, 1}
	} else if r == 0 && c == 1 {
		return []int{1, 0}
	} else if r == 1 && c == 0 {
		return []int{0, -1}
	} else if r == 0 && c == -1 {
		return []int{-1, 0}
	}
	return currentDir
}
