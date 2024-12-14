package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

func SolutionDay13part1(input []string) string {
	ans := 0
	coeff := make([]int, 0)
	equations := make([][]int, 0)
	costA, costB := 3, 1
	for _, line := range input {
		var x1, y1, x2, y2, x, y int
		if strings.Contains(line, "A") {
			x1 = getX(line)
			y1 = getY(line)
			coeff = append(coeff, x1, y1)
		} else if strings.Contains(line, "B") {
			x2 = getX(line)
			y2 = getY(line)
			coeff = append(coeff, x2, y2)
		} else if strings.Contains(line, "Prize") {
			x = getPrizeX(line)
			y = getPrizeY(line)
			coeff = append(coeff, x, y)
		} else {
			equations = append(equations, coeff)
			coeff = make([]int, 0)
			continue
		}
	}
	equations = append(equations, coeff)
	for _, coeff := range equations {
		x1 := coeff[0]
		x2 := coeff[2]
		x := coeff[4]
		y1 := coeff[1]
		y2 := coeff[3]
		y := coeff[5]
		integral, solution := solveEquation(x1, x2, x, y1, y2, y)
		if integral {
			ans += solution[0]*costA + solution[1]*costB
		}
		fmt.Println(integral, solution)
	}
	fmt.Println(equations)
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay13part2(input []string) string {
	ans := 0
	coeff := make([]int, 0)
	equations := make([][]int, 0)
	costA, costB := 3, 1
	for _, line := range input {
		var x1, y1, x2, y2, x, y int
		if strings.Contains(line, "A") {
			x1 = getX(line)
			y1 = getY(line)
			coeff = append(coeff, x1, y1)
		} else if strings.Contains(line, "B") {
			x2 = getX(line)
			y2 = getY(line)
			coeff = append(coeff, x2, y2)
		} else if strings.Contains(line, "Prize") {
			x = getPrizeX(line)
			y = getPrizeY(line)
			coeff = append(coeff, x, y)
		} else {
			equations = append(equations, coeff)
			coeff = make([]int, 0)
			continue
		}
	}
	equations = append(equations, coeff)
	for _, coeff := range equations {
		x1 := coeff[0]
		x2 := coeff[2]
		x := coeff[4] + 10000000000000
		y1 := coeff[1]
		y2 := coeff[3]
		y := coeff[5] + 10000000000000
		integral, solution := solveEquation(x1, x2, x, y1, y2, y)
		if integral {
			ans += solution[0]*costA + solution[1]*costB
		}
		fmt.Println(integral, solution)
	}
	fmt.Println(equations)
	return strconv.FormatInt(int64(ans), 10)
}

func getX(line string) int {
	aLine := strings.Split(line, ", ")
	aX := strings.Split(aLine[0], " ")
	xplus := strings.Split(aX[len(aX)-1], "+")
	x1, _ := strconv.Atoi(xplus[1])
	return x1
}

func getY(line string) int {
	aLine := strings.Split(line, ", ")
	aY := strings.Split(aLine[1], "+")
	y1, _ := strconv.Atoi(aY[1])
	return y1
}

func getPrizeX(line string) int {
	aLine := strings.Split(line, ", ")
	aX := strings.Split(aLine[0], "=")
	x, _ := strconv.Atoi(aX[len(aX)-1])
	return x
}
func getPrizeY(line string) int {
	aLine := strings.Split(line, ", ")
	aX := strings.Split(aLine[1], "=")
	x, _ := strconv.Atoi(aX[len(aX)-1])
	return x
}

func solveEquation(x1, x2, x, y1, y2, y int) (bool, []int) {
	numeratorA := (x2*y - x*y2)
	denominatorA := (x2*y1 - x1*y2)
	a := numeratorA / denominatorA
	numeratorB := (x - x1*a)
	denominatorB := x2
	b := numeratorB / denominatorB
	integral := numeratorA%denominatorA == 0 && numeratorB%denominatorB == 0

	return integral, []int{a, b}
}
