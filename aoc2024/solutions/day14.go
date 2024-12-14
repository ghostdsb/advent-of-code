package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

const GridX int = 101
const GridY int = 103

type Position struct {
	x int
	y int
}

type Velocity struct {
	x int
	y int
}

type Robot struct {
	position Position
	velocity Velocity
}

func SolutionDay14part1(input []string) string {
	ans := 0

	grid := make([][]int, 0)
	for y := 0; y < GridY; y++ {
		row := make([]int, 0)
		for x := 0; x < GridX; x++ {
			row = append(row, 0)
		}
		grid = append(grid, row)
	}

	fmt.Println()
	robots := make([]Robot, 0)
	for _, line := range input {
		robotStartX, robotStartY := parseRobotPosition(line)
		robotVelX, robotVelY := parseRobotVelocity(line)
		robots = append(robots, Robot{
			position: Position{x: robotStartX, y: robotStartY},
			velocity: Velocity{x: robotVelX, y: robotVelY},
		})
	}
	TimeSkip := 100
	for i := 0; i < TimeSkip; i++ {
		for j := 0; j < len(robots); j++ {
			moveRobot(robots, j)
			grid[robots[j].position.y][robots[j].position.x] += 1
		}
		if i < TimeSkip-1 {
			for y := 0; y < GridY; y++ {
				for x := 0; x < GridX; x++ {
					grid[y][x] = 0
				}
			}
		}
	}

	a := quad1Count(grid)
	b := quad2Count(grid)
	c := quad3Count(grid)
	d := quad4Count(grid)
	ans = a * b * c * d
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay14part2(input []string) string {
	grid := make([][]int, 0)
	for y := 0; y < GridY; y++ {
		row := make([]int, 0)
		for x := 0; x < GridX; x++ {
			row = append(row, 0)
		}
		grid = append(grid, row)
	}

	fmt.Println()
	robots := make([]Robot, 0)
	for _, line := range input {
		robotStartX, robotStartY := parseRobotPosition(line)
		robotVelX, robotVelY := parseRobotVelocity(line)
		robots = append(robots, Robot{
			position: Position{x: robotStartX, y: robotStartY},
			velocity: Velocity{x: robotVelX, y: robotVelY},
		})
	}

	for secondsElapsed := 1; ; secondsElapsed++ {
		for j := 0; j < len(robots); j++ {
			moveRobot(robots, j)
			grid[robots[j].position.y][robots[j].position.x] += 1
		}

		if hasCluster(grid) {
			for y := 0; y < GridY; y++ {
				for x := 0; x < GridX; x++ {
					if grid[y][x] == 0 {
						fmt.Printf("%s", ".")
					} else {
						fmt.Printf("%s", "#")
					}
				}
				fmt.Println()
			}
			return strconv.FormatInt(int64(secondsElapsed), 10)
		}
		for y := 0; y < GridY; y++ {
			for x := 0; x < GridX; x++ {
				grid[y][x] = 0
			}
		}
	}
}

func hasCluster(grid [][]int) bool {
	square := 3
	for i := 0; i < len(grid)-square; i++ {
		for j := 0; j < len(grid[i])-square; j++ {
			count := 0
			for k := 1; k <= square; k++ {
				for l := 1; l <= square; l++ {
					if grid[i+k][j+l] != 0 {
						count += 1
					}
				}
			}
			if count == (square * square) {
				return true
			}
		}
	}
	return false
}

func moveRobot(robots []Robot, i int) {
	robots[i].position.x = (robots[i].position.x + robots[i].velocity.x) % GridX
	robots[i].position.y = (robots[i].position.y + robots[i].velocity.y) % GridY

	if robots[i].position.x < 0 {
		robots[i].position.x += GridX
	}
	if robots[i].position.y < 0 {
		robots[i].position.y += GridY
	}
}

func parseRobotPosition(line string) (int, int) {
	robotInfoLine := strings.Split(line, " ")
	robotPosLine := strings.Split(robotInfoLine[0], "=")
	robotPosLine = strings.Split(robotPosLine[1], ",")
	robotPosX, _ := strconv.Atoi(robotPosLine[0])
	robotPosY, _ := strconv.Atoi(robotPosLine[1])
	return robotPosX, robotPosY
}

func parseRobotVelocity(line string) (int, int) {
	robotInfoLine := strings.Split(line, " ")
	robotPosLine := strings.Split(robotInfoLine[1], "=")
	robotPosLine = strings.Split(robotPosLine[1], ",")
	robotPosX, _ := strconv.Atoi(robotPosLine[0])
	robotPosY, _ := strconv.Atoi(robotPosLine[1])
	return robotPosX, robotPosY
}

func quad1Count(grid [][]int) int {
	ans := 0
	startX := len(grid[0])/2 + 1
	startY := 0
	endX := len(grid[0])
	endY := len(grid)/2 + 1
	for y := startY; y < endY; y++ {
		for x := startX; x < endX; x++ {
			ans += grid[y][x]
		}
	}
	fmt.Println(startX, startY, endX, endY)
	return ans
}

func quad2Count(grid [][]int) int {
	ans := 0
	startX := 0
	startY := 0
	endX := len(grid[0]) / 2
	endY := len(grid) / 2
	for y := startY; y < endY; y++ {
		for x := startX; x < endX; x++ {
			ans += grid[y][x]
		}
	}
	fmt.Println(startX, startY, endX, endY)
	return ans
}

func quad3Count(grid [][]int) int {
	ans := 0
	startX := 0
	startY := len(grid)/2 + 1
	endX := len(grid[0]) / 2
	endY := len(grid)
	for y := startY; y < endY; y++ {
		for x := startX; x < endX; x++ {
			ans += grid[y][x]
		}
	}
	fmt.Println(startX, startY, endX, endY)
	return ans
}

func quad4Count(grid [][]int) int {
	ans := 0
	startX := len(grid[0])/2 + 1
	startY := len(grid)/2 + 1
	endX := len(grid[0])
	endY := len(grid)
	for y := startY; y < endY; y++ {
		for x := startX; x < endX; x++ {
			ans += grid[y][x]
		}
	}
	fmt.Println(startX, startY, endX, endY)
	return ans
}

// ........................................#..................#.........................................
// .....................................................................................................
// ......................###############################................................................
// ......................#.............................#................................................
// ......................#.............................#................................................
// ......................#.............................#................................................
// ...................#..#.............................#................................................
// .#....................#..............#..............#................................................
// ......................#.............###.............#................................................
// .............#........#............#####............#................................................
// ......#......#...#....#...........#######...........#.......#.#......................................
// ......................#..........#########..........#.....#...............#................#.........
// ......................#............#####............#.............................#..................
// ...........#..........#...........#######...........#................................................
// ......................#..........#########..........#..........................#.....................
// ......................#.........###########.........#................................................
// ......................#........#############........#................................................
// ......................#..........#########..........#................#...............................
// ......................#.........###########.........#..................#.......#....#...........#....
// ......................#........#############........#......................#.........................
// ......................#.......###############.......#.....#..........................................
// ......................#......#################......#................................................
// ......................#........#############........#.......................................#........
// .#....................#.......###############.......#.............#...............................#..
// ..................#...#......#################......#................................................
// ...#.............#....#.....###################.....#................................................
// ......................#....#####################....#................................................
// ......................#.............###.............#.........................................#....#.
// ......................#.............###.............#..............................#.................
// ............#.......#.#.............###.............#................................................
// ......................#.............................#.................................#.....#........
// ......................#.............................#................................................
// ......................#.............................#................................................
// ......................#.............................#................................................
// ......................###############################................................................
// .............................................................................#.................#.....
// ................#.................................................................#..................
// .......................#....................................................#........................
