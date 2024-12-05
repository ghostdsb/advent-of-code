package utils

import (
	"bufio"
	"fmt"
	"os"
)

func ReadFile(filename string) ([]string, error) {
	file, err := os.Open(filename)
	if err != nil {
		fmt.Println("Error opening file", err)
	}

	defer file.Close()
	var lines []string

	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Error in reading file", err)
	}
	return lines, nil
}

func TopologicalSort(graph map[int][]int) []int {
	topOrder := make([]int, 0)
	inDegrees := make(map[int]int)

	for source, neighbours := range graph {
		if _, ok := inDegrees[source]; !ok {
			inDegrees[source] = 0
		}
		for _, num := range neighbours {
			inDegrees[num] += 1
		}
	}
	q := make([]int, 0)
	for k, v := range inDegrees {
		if v == 0 {
			q = append(q, k)
		}
	}
	for len(q) > 0 {
		node := q[0]
		topOrder = append(topOrder, node)
		q = q[1:]
		for _, neighbour := range graph[node] {
			inDegrees[neighbour] -= 1
			if inDegrees[neighbour] == 0 {
				q = append(q, neighbour)
			}
		}
	}
	return topOrder
}
