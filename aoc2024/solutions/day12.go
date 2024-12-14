package solutions

import (
	"fmt"
	"strconv"
)

type plot struct {
	r, c                     int
	fruit                    byte
	east, west, north, south bool
	visited                  bool
	southeast, southwest     bool
}

func (p plot) perimeter() int {
	total := 0
	if p.east {
		total += 1
	}
	if p.west {
		total += 1
	}
	if p.north {
		total += 1
	}
	if p.south {
		total += 1
	}
	return total
}

func (p plot) corner() int {
	total := 0
	if p.east && p.south {
		total += 1
	}
	if p.south && p.west {
		total += 1
	}
	if p.west && p.north {
		total += 1
	}
	if p.north && p.east {
		total += 1
	}

	if !p.east && p.south && !p.southeast {
		total += 1
	}
	if !p.west && p.south && !p.southwest {
		total += 1
	}

	if p.east && !p.south && !p.southeast {
		total += 1
	}
	if p.west && !p.south && !p.southwest {
		total += 1
	}
	return total
}

func SolutionDay12part1(input []string) string {
	ans := 0
	garden := make([][]plot, 0)
	for r, line := range input {
		rows := make([]plot, 0)
		for c, region := range line {
			plot := plot{
				r:         r,
				c:         c,
				east:      true,
				west:      true,
				north:     true,
				south:     true,
				fruit:     byte(region),
				visited:   false,
				southeast: true,
				southwest: true,
			}
			rows = append(rows, plot)
		}
		garden = append(garden, rows)
	}
	dirs := [][]int{
		{1, 0},
		{-1, 0},
		{0, 1},
		{0, -1},
	}
	for r, line := range garden {
		for c := range line {
			if !garden[r][c].visited {
				q := make([]plot, 0)
				q = append(q, garden[r][c])
				visits := make([][]int, 0)
				for len(q) > 0 {
					curr := q[0]
					visits = append(visits, []int{curr.r, curr.c})
					q = q[1:]
					garden[curr.r][curr.c].visited = true
					for _, dir := range dirs {
						nr := curr.r + dir[0]
						nc := curr.c + dir[1]
						if nr >= 0 && nc >= 0 && nr < len(garden) && nc < len(garden[0]) &&
							garden[curr.r][curr.c].fruit == garden[nr][nc].fruit {
							if !garden[nr][nc].visited {
								q = append(q, garden[nr][nc])
							}
							garden[nr][nc].visited = true
							if dir[0] == 1 && dir[1] == 0 {
								garden[curr.r][curr.c].south = false
								garden[nr][nc].north = false
							} else if dir[0] == -1 && dir[1] == 0 {
								garden[curr.r][curr.c].north = false
								garden[nr][nc].south = false
							} else if dir[0] == 0 && dir[1] == 1 {
								garden[curr.r][curr.c].east = false
								garden[nr][nc].west = false
							} else {
								garden[curr.r][curr.c].west = false
								garden[nr][nc].east = false
							}
						}
					}
				}
				fmt.Println(visits)
				perimeter := 0
				for _, visit := range visits {
					cell := garden[visit[0]][visit[1]]
					perimeter += cell.perimeter()
				}
				ans += perimeter * len(visits)
			}
		}
	}
	visualiseGarden(garden)
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay12part2(input []string) string {
	ans := 0
	garden := make([][]plot, 0)
	for r, line := range input {
		rows := make([]plot, 0)
		for c, region := range line {
			plot := plot{
				r:         r,
				c:         c,
				east:      true,
				west:      true,
				north:     true,
				south:     true,
				fruit:     byte(region),
				visited:   false,
				southeast: true,
				southwest: true,
			}
			rows = append(rows, plot)
		}
		garden = append(garden, rows)
	}
	dirs := [][]int{
		{1, 0},
		{-1, 0},
		{0, 1},
		{0, -1},
	}
	for r, line := range garden {
		for c := range line {
			if !garden[r][c].visited {
				q := make([]plot, 0)
				q = append(q, garden[r][c])
				visits := make([][]int, 0)
				for len(q) > 0 {
					curr := q[0]
					visits = append(visits, []int{curr.r, curr.c})
					q = q[1:]
					garden[curr.r][curr.c].visited = true
					for _, dir := range dirs {
						nr := curr.r + dir[0]
						nc := curr.c + dir[1]
						if nr >= 0 && nc >= 0 && nr < len(garden) && nc < len(garden[0]) &&
							garden[curr.r][curr.c].fruit == garden[nr][nc].fruit {
							if !garden[nr][nc].visited {
								q = append(q, garden[nr][nc])
							}
							garden[nr][nc].visited = true
							if dir[0] == 1 && dir[1] == 0 {
								garden[curr.r][curr.c].south = false
								garden[nr][nc].north = false
							} else if dir[0] == -1 && dir[1] == 0 {
								garden[curr.r][curr.c].north = false
								garden[nr][nc].south = false
							} else if dir[0] == 0 && dir[1] == 1 {
								garden[curr.r][curr.c].east = false
								garden[nr][nc].west = false
							} else {
								garden[curr.r][curr.c].west = false
								garden[nr][nc].east = false
							}
						}
					}
				}
				perimeter := 0
				corners := 0
				for _, visit := range visits {
					currRow := visit[0]
					currCol := visit[1]
					diagonalR := currRow + 1
					leftC := currCol - 1
					rightC := currCol + 1

					if diagonalR < len(garden) {
						if leftC >= 0 {
							if garden[currRow][currCol].fruit == garden[diagonalR][leftC].fruit {
								garden[currRow][currCol].southwest = false
							}
						}
						if rightC < len(garden[currRow]) {
							if garden[currRow][currCol].fruit == garden[diagonalR][rightC].fruit {
								garden[currRow][currCol].southeast = false
							}
						}

					}
					cell := garden[currRow][currCol]
					perimeter += cell.perimeter()
					corners += cell.corner()
				}
				fmt.Printf("%s %d %d\n", string(garden[r][c].fruit), corners, len(visits))
				ans += corners * len(visits)
			}
		}
	}

	for _, line := range garden {
		for _, region := range line {
			fmt.Printf("%s E:%t SE:%t S:%t SW:%t W:%t N:%t ", string(region.fruit), region.east, region.southeast, region.south, region.southwest, region.west, region.north)
		}
		fmt.Println()
	}
	return strconv.FormatInt(int64(ans), 10)
}

func visualiseGarden(garden [][]plot) {
	for _, line := range garden {
		for i := 0; i < 3; i++ {
			for _, region := range line {
				if i == 0 {
					if region.north {
						fmt.Printf("%s", " - ")
					} else {
						fmt.Printf("%s", "   ")
					}
				} else if i == 1 {
					if region.west {
						fmt.Printf("%s", "|")
					} else {
						fmt.Printf("%s", " ")
					}
					fmt.Printf("%s", string(region.fruit))
					if region.east {
						fmt.Printf("%s", "|")
					} else {
						fmt.Printf("%s", " ")
					}
				} else {
					if region.south {
						fmt.Printf("%s", " - ")
					} else {
						fmt.Printf("%s", "   ")
					}
				}
			}
			fmt.Println()
		}
	}
}
