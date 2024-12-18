package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

type Path struct {
	tile  Tile
	depth int
}

func SolutionDay18(input []string) string {
	gridSize := 71
	pointsRange := 1024
	points := make([]Point, 0)
	for _, line := range input {
		coords := strings.Split(line, ",")
		x, _ := strconv.Atoi(coords[0])
		y, _ := strconv.Atoi(coords[1])
		points = append(points, Point{x, y})
	}
	grid := make([][]Tile, 0)
	for i := 0; i < gridSize; i++ {
		row := make([]Tile, 0)
		for j := 0; j < gridSize; j++ {
			row = append(row, Tile{x: j, y: i, tileType: '.'})
		}
		grid = append(grid, row)
	}
	mz := make([][]Tile, len(grid))
	for i := range grid {
		mz[i] = make([]Tile, len(grid[i]))
		copy(mz[i], grid[i])
	}
	part1 := d18p1(mz, points, gridSize, pointsRange)
	part2 := d18p2(grid, points, gridSize)

	return fmt.Sprintf("%d %s", part1, part2)
}

func d18p1(grid [][]Tile, points []Point, gridSize, pointsRange int) int {
	ans := 0
	for _, point := range points[0:pointsRange] {
		grid[point.y][point.x].tileType = '#'
	}
	startX, startY := 0, 0
	endX, endY := gridSize-1, gridSize-1

	q := make([]Path, 0)
	dir := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	q = append(q, Path{depth: 0, tile: grid[startY][startX]})
	for len(q) > 0 {
		top := q[0]
		q = q[1:]
		if top.tile.x == endX && top.tile.y == endY {
			ans = top.depth
			return ans
		}
		grid[top.tile.y][top.tile.x].tileType = 'O'
		for _, d := range dir {
			nx := top.tile.x + d[1]
			ny := top.tile.y + d[0]
			if nx >= 0 && ny >= 0 && nx < gridSize && ny < gridSize && grid[ny][nx].tileType == '.' {
				q = append(q, Path{depth: top.depth + 1, tile: grid[ny][nx]})
				grid[ny][nx].tileType = '0'
			}
		}
	}
	return -1
}

func d18p2(grid [][]Tile, points []Point, gridSize int) string {

	for i := 1; i < len(points); i++ {
		mz := make([][]Tile, len(grid))
		for j := range grid {
			mz[j] = make([]Tile, len(grid[j]))
			copy(mz[j], grid[j])
		}

		found := d18p1(mz, points, gridSize, i)
		if found == -1 {
			return fmt.Sprintf("%d,%d", points[i-1].x, points[i-1].y)
		}
	}
	return ""
}
