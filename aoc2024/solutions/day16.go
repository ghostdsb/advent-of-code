package solutions

import (
	"fmt"
	"math"
	"strconv"
)

type Tile struct {
	x, y     int
	tileType rune
	score    int
	from     *Tile
	facing   int
	fromList []*Tile
}

func SolutionDay16(input []string) string {

	maze := make([][]Tile, 0)
	for r, line := range input {
		row := make([]Tile, 0)
		for c, tile := range line {
			t := Tile{x: c, y: r, score: -1, from: nil, tileType: tile, facing: -1}
			row = append(row, t)
		}
		maze = append(maze, row)
	}
	visualiseMaze(maze)

	maze[len(maze)-2][1].facing = 0
	maze[len(maze)-2][1].score = 0

	mz := make([][]Tile, len(maze))
	for i := range maze {
		mz[i] = make([]Tile, len(maze[i]))
		copy(mz[i], maze[i])
	}

	part1 := d16p1(mz)
	part2 := d16p2(maze)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d16p1(maze [][]Tile) string {
	ans := 0
	startX, startY := 1, len(maze)-2
	endX, endY := len(maze[1])-2, 1
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
	for len(queue) > 0 {
		top := queue[0]
		queue = queue[1:]
		if top.tileType == 'E' {
			fmt.Printf("%+v\n", top)
			continue
		}
		for _, d := range dir {
			nx := top.x + d[0]
			ny := top.y + d[1]
			if maze[ny][nx].tileType == '.' || maze[ny][nx].tileType == 'E' {
				score := top.score + 1
				facing := -1
				if d[0] == 0 && d[1] == 1 {
					if top.facing != 1 {
						score += 1000
					}
					facing = 1
				}
				if d[0] == 0 && d[1] == -1 {
					if top.facing != 3 {
						score += 1000
					}
					facing = 3
				}
				if d[0] == 1 && d[1] == 0 {
					if top.facing != 0 {
						score += 1000
					}
					facing = 0
				}
				if d[0] == -1 && d[1] == 0 {
					if top.facing != 2 {
						score += 1000
					}
					facing = 2
				}
				if maze[ny][nx].score == -1 || score < maze[ny][nx].score {
					maze[ny][nx].score = score
					maze[ny][nx].facing = facing
					maze[ny][nx].from = &top
					queue = append(queue, maze[ny][nx])
				}
			}
		}
	}
	for _, r := range maze {
		for _, c := range r {
			fmt.Printf(" %05d", c.score)
		}
		fmt.Println()
	}
	curr := maze[endY][endX]
	for curr.tileType != 'S' {
		x, y := curr.from.x, curr.from.y
		maze[curr.y][curr.x].tileType = 'O'
		curr = maze[y][x]
	}
	visualiseMaze(maze)
	ans = maze[1][len(maze)-2].score
	return strconv.FormatInt(int64(ans), 10)
}

func d16p2(maze [][]Tile) string {
	ans := 0
	startX, startY := 1, len(maze)-2
	endX, endY := len(maze[1])-2, 1
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
	for len(queue) > 0 {
		top := queue[0]
		queue = queue[1:]
		if top.tileType == 'E' {
			fmt.Printf("m%+v\n", top)
			continue
		}
		for _, d := range dir {
			nx := top.x + d[0]
			ny := top.y + d[1]
			if maze[ny][nx].tileType == '.' || maze[ny][nx].tileType == 'E' {
				score := top.score + 1
				facing := -1
				if d[0] == 0 && d[1] == 1 {
					if top.facing != 1 {
						score += 1000
					}
					facing = 1
				}
				if d[0] == 0 && d[1] == -1 {
					if top.facing != 3 {
						score += 1000
					}
					facing = 3
				}
				if d[0] == 1 && d[1] == 0 {
					if top.facing != 0 {
						score += 1000
					}
					facing = 0
				}
				if d[0] == -1 && d[1] == 0 {
					if top.facing != 2 {
						score += 1000
					}
					facing = 2
				}
				if maze[ny][nx].score == -1 || score < maze[ny][nx].score || score == maze[ny][nx].score+1000 {
					maze[ny][nx].score = score
					maze[ny][nx].facing = facing
					maze[ny][nx].from = &top
					l := make([]*Tile, 0)
					for _, n := range maze[ny][nx].fromList {
						if math.Abs(float64(n.score)-float64(score)) <= 1000 {
							l = append(l, n)
						}
					}
					l = append(l, &top)
					maze[ny][nx].fromList = l
					queue = append(queue, maze[ny][nx])
				}
			}
		}
	}
	// for _, r := range maze {
	// 	for _, c := range r {
	// 		fmt.Printf(" %05d", c.score)
	// 	}
	// 	fmt.Println()
	// }
	curr := maze[endY][endX]
	q := make([]Tile, 0)
	q = append(q, curr)
	visited := make(map[string]bool)
	for len(q) > 0 {
		top := q[0]
		fmt.Printf("%d-%d\n", top.y, top.x)
		q = q[1:]
		for _, tile := range top.fromList {
			// if tile != nil {
			if _, ok := visited[fmt.Sprintf("%d-%d", top.x, top.y)]; !ok {
				q = append(q, *tile)
				ans += 1
				visited[fmt.Sprintf("%d-%d", top.x, top.y)] = true
			}
			// }
		}
		// if top.tileType == 'S' {
		// 	break
		// }
	}
	// for curr.tileType != 'S' {
	// 	x, y := curr.from.x, curr.from.y
	// 	maze[curr.y][curr.x].tileType = 'O'
	// 	curr = maze[y][x]
	// }
	fmt.Printf("%+v %p\n", maze[7][5], &maze[7][5])
	for _, t := range maze[7][5].fromList {
		fmt.Println(t)
	}
	// fmt.Printf("%+v %p\n", maze[7][4], &maze[7][4])
	// fmt.Printf("%+v %p\n", maze[8][5], &maze[8][5])
	visualiseMaze(maze)
	return strconv.FormatInt(int64(ans), 10)
}

func visualiseMaze(maze [][]Tile) {
	for _, line := range maze {
		for _, entity := range line {
			fmt.Printf("%s", string(entity.tileType))
		}
		fmt.Println()
	}
}
