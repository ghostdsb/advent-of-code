package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

type operation struct {
	input1    string
	input2    string
	output    string
	operation string
}

func (o operation) operate(wires map[string]int) int {
	if o.operation == "AND" {
		return wires[o.input1] & wires[o.input2]
	}
	if o.operation == "OR" {
		return wires[o.input1] | wires[o.input2]
	}
	if o.operation == "XOR" {
		return wires[o.input1] ^ wires[o.input2]
	}
	return 0
}

func SolutionDay24(input []string) string {

	wires := make(map[string]int)
	operations := make([]operation, 0)
	allWiresParsed := false
	for _, line := range input {
		if !allWiresParsed {
			if line == "" {
				allWiresParsed = true
			} else {
				comps := strings.Split(line, ": ")
				val, _ := strconv.Atoi(comps[1])
				wires[comps[0]] = val
			}
		} else {
			comps := strings.Split(line, " ")
			operations = append(operations, operation{comps[0], comps[2], comps[4], comps[1]})
		}
	}
	fmt.Println(wires)
	fmt.Println(operations)
	part1 := d24p1(wires, operations)
	part2 := d24p2(input)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d24p1(wires map[string]int, operations []operation) string {
	var ans int64
	zWireCount := 0
	for _, operation := range operations {
		fmt.Println(operation)
		if strings.HasPrefix(operation.output, "z") {
			zWireCount += 1
		}
	}

	for i := 0; i < 7; i++ {
		for _, operation := range operations {
			output := operation.operate(wires)
			wires[operation.output] = output
		}
	}

	fmt.Println(wires)
	for wire, value := range wires {
		fmt.Printf("%s: %d\n", wire, value)
	}
	nameLength := max(2, len(strconv.Itoa(zWireCount)))
	binary := ""
	for i := zWireCount - 1; i >= 0; i-- {
		name := fmt.Sprintf("z%0*d", nameLength, i)
		binary += strconv.Itoa(wires[name])
	}
	fmt.Println(binary)
	bin, _ := strconv.ParseInt(binary, 2, 64)
	ans = bin
	return strconv.FormatInt(int64(ans), 10)
}

func d24p2(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}
