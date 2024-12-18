package solutions

import (
	"fmt"
	"strconv"
)

type Entity struct {
	x, y    int
	item    byte
	moveDir byte
}

func SolutionDay15(input []string) string {

	warehouse := make([][]Entity, 0)
	directions := make([]byte, 0)
	startX, startY := 0, 0

	dirParsingStart := false
	for y, line := range input {
		if line == "" {
			dirParsingStart = true
		}
		row := make([]Entity, 0)
		if !dirParsingStart {
			for x, region := range line {
				entity := Entity{x: x, y: y, item: byte(region), moveDir: '0'}
				row = append(row, entity)
				if entity.item == '@' {
					startX, startY = x, y

				}
			}
			warehouse = append(warehouse, row)
		} else {
			if line != "" {
				for _, dir := range line {
					directions = append(directions, byte(dir))
				}

			}
		}
	}
	w := make([][]Entity, len(warehouse))
	for i := range warehouse {
		w[i] = make([]Entity, len(warehouse[i]))
		copy(w[i], warehouse[i])
	}
	part1 := d15p1(w, directions, startX, startY)
	part2 := d15p2(warehouse, directions, startX, startY)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d15p1(warehouse [][]Entity, directions []byte, x, y int) string {
	ans := 0
	// visualiseWarehouse(warehouse)
	for _, dir := range directions {
		if dir == '<' {
			x, y = entitiesMoveLeft(warehouse, x, y)
		}
		if dir == '>' {
			x, y = entitiesMoveRight(warehouse, x, y)
		}
		if dir == '^' {
			x, y = entitiesMoveUp(warehouse, x, y)
		}
		if dir == 'v' {
			x, y = entitiesMoveDown(warehouse, x, y)
		}
	}
	for r, line := range warehouse {
		for c, entity := range line {
			if entity.item == 'O' {
				ans += r*100 + c
			}
		}
	}
	visualiseWarehouse(warehouse)
	return strconv.FormatInt(int64(ans), 10)
}

func d15p2(warehouse [][]Entity, directions []byte, x, y int) string {
	ans := 0
	bigWarehouse := make([][]Entity, 0)
	for r, line := range warehouse {
		row := make([]Entity, 0)
		for c, entity := range line {
			if entity.item == 'O' {
				entity1 := Entity{x: 2 * c, y: r, moveDir: 0, item: '['}
				entity2 := Entity{x: entity.x + 1, y: r, moveDir: 0, item: ']'}
				row = append(row, []Entity{entity1, entity2}...)
			} else if entity.item == '.' {
				entity1 := Entity{x: 2 * c, y: r, moveDir: 0, item: '.'}
				entity2 := Entity{x: entity.x + 1, y: r, moveDir: 0, item: '.'}
				row = append(row, []Entity{entity1, entity2}...)

			} else if entity.item == '@' {
				entity1 := Entity{x: 2 * c, y: r, moveDir: 0, item: '@'}
				entity2 := Entity{x: entity.x + 1, y: r, moveDir: 0, item: '.'}
				row = append(row, []Entity{entity1, entity2}...)
			} else if entity.item == '#' {
				entity1 := Entity{x: 2 * c, y: r, moveDir: 0, item: '#'}
				entity2 := Entity{x: entity.x + 1, y: r, moveDir: 0, item: '#'}
				row = append(row, []Entity{entity1, entity2}...)
			}
		}
		bigWarehouse = append(bigWarehouse, row)
	}
	x *= 2
	// visualiseWarehouse(bigWarehouse)
	fmt.Println()
	for _, dir := range directions {
		if dir == '<' {
			x, y = entitiesMoveLeft(bigWarehouse, x, y)
		}
		if dir == '>' {
			x, y = entitiesMoveRight(bigWarehouse, x, y)
		}
		if dir == '^' {
			x, y = entitiesMoveUp2(bigWarehouse, x, y)
		}
		if dir == 'v' {
			x, y = entitiesMoveDown2(bigWarehouse, x, y)
		}
	}
	visualiseWarehouse(bigWarehouse)
	for r, line := range bigWarehouse {
		for c, entity := range line {
			if entity.item == '[' {
				ans += r*100 + c
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func entitiesMoveLeft(warehouse [][]Entity, robotX, robotY int) (int, int) {
	rails := make([][]int, 0)
	x, y := robotX, robotY
	for i := robotX; i >= 0; i-- {
		if warehouse[robotY][i].item == '#' {
			rails = make([][]int, 0)
			break
		}
		rails = append(rails, []int{robotY, i})
		if warehouse[robotY][i].item == '.' {
			x -= 1
			break
		}
	}
	for i := len(rails) - 1; i > 0; i-- {
		a := rails[i][0]
		b := rails[i][1]

		c := rails[i-1][0]
		d := rails[i-1][1]

		warehouse[a][b], warehouse[c][d] = warehouse[c][d], warehouse[a][b]
	}
	return x, y
}
func entitiesMoveRight(warehouse [][]Entity, robotX, robotY int) (int, int) {
	rails := make([][]int, 0)
	x, y := robotX, robotY
	for i := robotX; i < len(warehouse[robotY]); i++ {
		if warehouse[robotY][i].item == '#' {
			rails = make([][]int, 0)
			break
		}
		rails = append(rails, []int{robotY, i})
		if warehouse[robotY][i].item == '.' {
			x += 1
			break
		}
	}
	for i := len(rails) - 1; i > 0; i-- {
		a := rails[i][0]
		b := rails[i][1]

		c := rails[i-1][0]
		d := rails[i-1][1]

		warehouse[a][b], warehouse[c][d] = warehouse[c][d], warehouse[a][b]
	}
	return x, y
}
func entitiesMoveUp(warehouse [][]Entity, robotX, robotY int) (int, int) {
	rails := make([][]int, 0)
	x, y := robotX, robotY
	for i := robotY; i >= 0; i-- {
		if warehouse[i][robotX].item == '#' {
			rails = make([][]int, 0)
			break
		}
		rails = append(rails, []int{i, robotX})
		if warehouse[i][robotX].item == '.' {
			y -= 1
			break
		}
	}
	for i := len(rails) - 1; i > 0; i-- {
		a := rails[i][0]
		b := rails[i][1]

		c := rails[i-1][0]
		d := rails[i-1][1]

		warehouse[a][b], warehouse[c][d] = warehouse[c][d], warehouse[a][b]
	}
	return x, y
}
func entitiesMoveUp2(warehouse [][]Entity, robotX, robotY int) (int, int) {
	x, y := robotX, robotY
	visited := make([][]int, 0)
	stack := make([][]int, 0)
	dict := make(map[string]bool)
	stack = append(stack, []int{y, x})
	for len(stack) > 0 {
		top := stack[0]
		stack = stack[1:]
		visited = append(visited, []int{top[0], top[1]})
		dict[fmt.Sprintf("%d-%d", top[0], top[1])] = true
		topY := top[0] - 1
		topX := top[1]
		if warehouse[topY][topX].item == '[' {
			if _, ok := dict[fmt.Sprintf("%d-%d", topY, topX)]; !ok {
				stack = append(stack, []int{topY, topX})
				dict[fmt.Sprintf("%d-%d", topY, topX)] = true
			}
			if _, ok := dict[fmt.Sprintf("%d-%d", topY, topX+1)]; !ok {
				stack = append(stack, []int{topY, topX + 1})
				dict[fmt.Sprintf("%d-%d", topY, topX+1)] = true
			}
		}
		if warehouse[topY][topX].item == ']' {
			if _, ok := dict[fmt.Sprintf("%d-%d", topY, topX)]; !ok {
				stack = append(stack, []int{topY, topX})
				dict[fmt.Sprintf("%d-%d", topY, topX)] = true
			}
			if _, ok := dict[fmt.Sprintf("%d-%d", topY, topX-1)]; !ok {
				stack = append(stack, []int{topY, topX - 1})
				dict[fmt.Sprintf("%d-%d", topY, topX-1)] = true
			}
		}
		if warehouse[topY][topX].item == '#' {
			visited = make([][]int, 0)
			break
		}
	}
	// fmt.Println(visited)
	for i := len(visited) - 1; i >= 0; i-- {
		if i == 0 {
			y -= 1
		}
		a := visited[i][0]
		b := visited[i][1]

		warehouse[a-1][b] = warehouse[a][b]
		warehouse[a][b] = Entity{x: b, y: a, item: '.'}
		// visualiseWarehouse(warehouse)
	}
	return x, y
}
func entitiesMoveDown(warehouse [][]Entity, robotX, robotY int) (int, int) {
	rails := make([][]int, 0)
	x, y := robotX, robotY
	for i := robotY; i < len(warehouse); i++ {
		if warehouse[i][robotX].item == '#' {
			rails = make([][]int, 0)
			break
		}
		rails = append(rails, []int{i, robotX})
		if warehouse[i][robotX].item == '.' {
			y += 1
			break
		}
	}
	for i := len(rails) - 1; i > 0; i-- {
		a := rails[i][0]
		b := rails[i][1]

		c := rails[i-1][0]
		d := rails[i-1][1]

		warehouse[a][b], warehouse[c][d] = warehouse[c][d], warehouse[a][b]
	}
	return x, y
}

func entitiesMoveDown2(warehouse [][]Entity, robotX, robotY int) (int, int) {
	x, y := robotX, robotY
	visited := make([][]int, 0)
	stack := make([][]int, 0)
	dict := make(map[string]bool)
	stack = append(stack, []int{y, x})
	for len(stack) > 0 {
		top := stack[0]
		stack = stack[1:]
		visited = append(visited, []int{top[0], top[1]})
		dict[fmt.Sprintf("%d-%d", top[0], top[1])] = true
		bottomY := top[0] + 1
		bottomX := top[1]
		if warehouse[bottomY][bottomX].item == '[' {
			if _, ok := dict[fmt.Sprintf("%d-%d", bottomY, bottomX)]; !ok {
				stack = append(stack, []int{bottomY, bottomX})
				dict[fmt.Sprintf("%d-%d", bottomY, bottomX)] = true
			}
			if _, ok := dict[fmt.Sprintf("%d-%d", bottomY, bottomX+1)]; !ok {
				stack = append(stack, []int{bottomY, bottomX + 1})
				dict[fmt.Sprintf("%d-%d", bottomY, bottomX+1)] = true
			}
		}
		if warehouse[bottomY][bottomX].item == ']' {
			if _, ok := dict[fmt.Sprintf("%d-%d", bottomY, bottomX)]; !ok {
				stack = append(stack, []int{bottomY, bottomX})
				dict[fmt.Sprintf("%d-%d", bottomY, bottomX)] = true
			}
			if _, ok := dict[fmt.Sprintf("%d-%d", bottomY, bottomX-1)]; !ok {
				stack = append(stack, []int{bottomY, bottomX - 1})
				dict[fmt.Sprintf("%d-%d", bottomY, bottomX-1)] = true
			}
		}
		if warehouse[bottomY][bottomX].item == '#' {
			visited = make([][]int, 0)
			break
		}
	}
	// fmt.Println(visited)
	for i := len(visited) - 1; i >= 0; i-- {
		if i == 0 {
			y += 1
		}
		a := visited[i][0]
		b := visited[i][1]

		warehouse[a+1][b] = warehouse[a][b]
		warehouse[a][b] = Entity{x: b, y: a, item: '.'}
		// visualiseWarehouse(warehouse)
	}
	return x, y
}
func visualiseWarehouse(warehouse [][]Entity) {
	for _, line := range warehouse {
		for _, entity := range line {
			fmt.Printf("%s", string(entity.item))
		}
		fmt.Println()
	}
}
