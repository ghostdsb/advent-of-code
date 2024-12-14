package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

func SolutionDay11part1(input []string) string {
	ans := 0
	stones := make([]int, 0)
	for _, s := range strings.Fields(input[0]) {
		st, _ := strconv.Atoi(string(s))
		stones = append(stones, st)
	}
	blinkCount := 25
	for i := 0; i < blinkCount; i++ {
		run := make([]int, 0)
		for _, stone := range stones {
			x := rule(stone)
			run = append(run, x...)
		}
		stones = run
	}

	ans = len(stones)
	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay11part2(input []string) string {
	ans := 0
	stones := make([]int, 0)
	for _, s := range strings.Fields(input[0]) {
		st, _ := strconv.Atoi(string(s))
		stones = append(stones, st)
	}
	blinkCount := 2

	ans = getStoneCountAfterBlinking(stones, blinkCount)
	return strconv.FormatInt(int64(ans), 10)
}

func getStoneCountAfterBlinking(input []int, timesBlink int) int {
	sum := 0
	cache := make(map[int][]int)
	for _, stone := range input {
		sum += getCountAfterBlinks(stone, cache, timesBlink)
	}
	fmt.Printf("%+v\n", cache)
	return sum
}

func getCountAfterBlinks(stone int, cache map[int][]int, blinkCount int) int {
	if _, ok := cache[stone]; ok {
		if cache[stone][blinkCount-1] != 0 {
			return cache[stone][blinkCount-1]
		}
	} else {
		cache[stone] = make([]int, 75)
	}

	if blinkCount == 1 {
		cache[stone][blinkCount-1] = len(rule(stone))
		return len(rule(stone))
	}

	sum := 0

	for _, stone := range rule(stone) {
		sum += getCountAfterBlinks(stone, cache, blinkCount-1)
	}

	cache[stone][blinkCount-1] = sum
	return sum
}

func rule(stone int) []int {
	numStr := fmt.Sprintf("%d", stone)
	new := make([]int, 0)
	if stone == 0 {
		new = append(new, 1)
	} else if len(numStr)%2 == 0 {
		l := len(numStr)
		a, _ := strconv.Atoi(numStr[:l/2])
		b, _ := strconv.Atoi(numStr[l/2:])
		new = append(new, a)
		new = append(new, b)
	} else {
		new = append(new, stone*2024)
	}
	return new
}
