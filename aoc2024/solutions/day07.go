package solutions

import (
	"strconv"
	"strings"
)

func SolutionDay07part1(input []string) string {
	ans := 0
	for _, line := range input {
		equation := strings.Split(line, ":")
		target, _ := strconv.Atoi(equation[0])
		vals := strings.Fields(equation[1])
		values := make([]int, 0)
		for _, val := range vals {
			v, _ := strconv.Atoi(val)
			values = append(values, v)
		}
		results := []int{}

		insertOperators(target, values, []byte{}, &results, 0)
		for _, r := range results {
			if r == target {
				ans += target
				break
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay07part2(input []string) string {
	ans := 0
	for _, line := range input {
		equation := strings.Split(line, ":")
		target, _ := strconv.Atoi(equation[0])
		vals := strings.Fields(equation[1])
		values := make([]int, 0)
		for _, val := range vals {
			v, _ := strconv.Atoi(val)
			values = append(values, v)
		}
		results := []int{}

		insertOperators2(target, values, []byte{}, &results, 0)
		for _, r := range results {
			if r == target {
				ans += target
				break
			}
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func insertOperators(target int, numbers []int, current []byte, results *[]int, idx int) {
	if idx == len(numbers)-1 {
		result := evaluateExpression(numbers, current)
		*results = append(*results, result)
		return
	}

	current = append(current, '+')
	insertOperators(target, numbers, current, results, idx+1)
	current = current[:len(current)-1] // Backtrack

	current = append(current, 'x')
	insertOperators(target, numbers, current, results, idx+1)
}

func insertOperators2(target int, numbers []int, current []byte, results *[]int, idx int) {
	if idx == len(numbers)-1 {
		result := evaluateExpression(numbers, current)
		*results = append(*results, result)
		return
	}

	current = append(current, '+')
	insertOperators2(target, numbers, current, results, idx+1)
	current = current[:len(current)-1] // Backtrack

	current = append(current, 'x')
	insertOperators2(target, numbers, current, results, idx+1)
	current = current[:len(current)-1] // Backtrack

	current = append(current, '|')
	insertOperators2(target, numbers, current, results, idx+1)
}

func evaluateExpression(numbers []int, operators []byte) int {
	result := numbers[0]
	for i, op := range operators {
		if op == '+' {
			result += numbers[i+1]
		} else if op == 'x' {
			result *= numbers[i+1]
		} else if op == '|' {
			resultStr := strconv.FormatInt(int64(result), 10)
			numberStr := strconv.FormatInt(int64(numbers[i+1]), 10)
			res := resultStr + numberStr
			result, _ = strconv.Atoi(res)
		}
	}
	return result
}
