package solutions

import (
	"fmt"
)

func SolutionDay20(input []string) string {
	maze := make([][]Tile, 0)
	for r, line := range input {
		row := make([]Tile, 0)
		for c, tile := range line {
			t := Tile{x: c, y: r, score: 0, from: nil, tileType: tile, facing: -1}
			row = append(row, t)
		}
		maze = append(maze, row)
	}

	maze[len(maze)-2][1].facing = 0
	maze[len(maze)-2][1].score = 0

	mz := make([][]Tile, len(maze))
	for i := range maze {
		mz[i] = make([]Tile, len(maze[i]))
		copy(mz[i], maze[i])
	}

	part1 := solve(mz, 2)   // 1381
	part2 := solve(maze, 6) //982124

	return fmt.Sprintf("%d %d", part1, part2)
}

func shortCirtcuitCount(maze [][]Tile, tile Tile, distance int) []int {
	count := 0
	saves := make([]int, 0)
	for dy := -distance; dy <= distance; dy++ {
		for dx := -distance; dx <= distance; dx++ {
			nx := tile.x + dx
			ny := tile.y + dy
			manhattan := abs(nx-tile.x) + abs(ny-tile.y)
			if nx >= 0 && ny >= 0 && nx < len(maze[0]) && ny < len(maze) &&
				manhattan <= distance {
				if (maze[ny][nx].tileType == '.' || maze[ny][nx].tileType == 'E' || maze[ny][nx].tileType == 'S') &&
					abs(maze[ny][nx].score-tile.score) > manhattan {
					// fmt.Printf("x %+v %+v\n", tile, maze[ny][nx])
					count += 1
					saves = append(saves, abs(maze[ny][nx].score-tile.score))
				}
			}
		}
	}
	maze[tile.y][tile.x].tileType = 'O'
	return saves
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func traverse(maze [][]Tile) []Tile {
	startX, startY := -1, -1
	endX, endY := -1, -1
	for r, line := range maze {
		for c, t := range line {
			if t.tileType == 'S' {
				startX, startY = c, r
			}
			if t.tileType == 'E' {
				endX, endY = c, r

			}
		}
	}
	dir := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	queue := make([]Tile, 0)
	queue = append(queue, maze[startY][startX])
	visited := make(map[string]bool)
	for len(queue) > 0 {
		top := queue[0]
		queue = queue[1:]
		visited[fmt.Sprintf("%d,%d", top.x, top.y)] = true
		if top.tileType == 'E' {
			continue
		}
		for _, d := range dir {
			nx := top.x + d[0]
			ny := top.y + d[1]
			if maze[ny][nx].tileType == '.' || maze[ny][nx].tileType == 'E' {
				if _, ok := visited[fmt.Sprintf("%d,%d", nx, ny)]; !ok {
					maze[ny][nx].from = &top
					maze[ny][nx].score = top.score + 1
					queue = append(queue, maze[ny][nx])
					visited[fmt.Sprintf("%d,%d", nx, ny)] = true
				}
			}
		}
	}
	path := make([]Tile, 0)
	curr := maze[endY][endX]
	for curr.tileType != 'S' {
		x, y := curr.from.x, curr.from.y
		path = append(path, curr)
		curr = maze[y][x]
	}
	path = append(path, curr)
	return path
}

func solve(maze [][]Tile, cheatCount int) int {
	path := traverse(maze)
	freqMap := make(map[int]int)
	for _, square := range path {
		sc := shortCirtcuitCount(maze, square, cheatCount)
		for _, s := range sc {
			freqMap[s] += 1
		}
	}
	fmt.Println(freqMap)
	ans := 0
	for k, v := range freqMap {
		if abs(k) >= 100 {
			ans += v
		}
	}
	return ans
}
