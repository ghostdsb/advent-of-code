package solutions

import (
	"fmt"
	"strconv"
)

func SolutionDay25(input []string) string {

	keysGrids := make([][][]rune, 0)
	lockGrids := make([][][]rune, 0)
	isKey := true
	shouldDecideType := true
	grid := make([][]rune, 0)
	for _, line := range input {
		row := make([]rune, 0)
		if line != "" {
			if shouldDecideType {
				if line[0] == '.' {
					isKey = true
				} else {
					isKey = false
				}
				shouldDecideType = false
			}
			for _, point := range line {
				row = append(row, point)
			}
			grid = append(grid, row)
		} else {
			if isKey {
				keysGrids = append(keysGrids, grid)
			} else {
				lockGrids = append(lockGrids, grid)
			}
			shouldDecideType = true
			grid = make([][]rune, 0)
		}
	}
	if isKey {
		keysGrids = append(keysGrids, grid)
	} else {
		lockGrids = append(lockGrids, grid)
	}

	// fmt.Println(lockGrids)
	// fmt.Println(keysGrids)

	for _, grid := range keysGrids {
		printGrid(grid)
		fmt.Println()
	}
	for _, grid := range lockGrids {
		printGrid(grid)
		fmt.Println()
	}

	key1 := gridToEntity(keysGrids[0])
	fmt.Println(key1)

	ans := 0

	for _, key := range keysGrids {
		keyEntity := gridToEntity(key)
		for _, lock := range lockGrids {
			lockEntity := gridToEntity(lock)
			fit := true
			for i := 0; i < len(keyEntity); i++ {
				if keyEntity[i]+lockEntity[i] > 5 {
					fit = false
					break
				}
			}
			if fit {
				ans += 1
			}
		}
	}

	fmt.Println(ans)

	part1 := d25p1(input)
	part2 := d25p2(input)

	return fmt.Sprintf("%s %s", part1, part2)
}

func printGrid(grid [][]rune) {
	for _, line := range grid {
		for _, p := range line {
			fmt.Printf("%c ", p)
		}
		fmt.Println()
	}
}

func gridToEntity(grid [][]rune) []int {
	key := make([]int, len(grid[0]))
	printGrid(grid)
	for j := 0; j < len(grid[0]); j++ {
		count := 0
		for i := 1; i < len(grid)-1; i++ {
			if grid[i][j] == '#' {
				count += 1
			}
		}
		key[j] = count
	}
	return key
}

func d25p1(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}

func d25p2(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}
