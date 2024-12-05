package solutions

import (
	"strconv"
	"strings"

	"golang.org/x/exp/slices"
)

func SolutionDay05part1(input []string) string {
	ans := 0
	beforeRule := make(map[string]map[string]struct{})
	afterRule := make(map[string]map[string]struct{})

	sort := func(a, b string) int {
		if _, ok := beforeRule[a][b]; ok {
			return 1
		}
		if _, ok := afterRule[a][b]; ok {
			return -1
		}
		return 0
	}

	for _, line := range input {
		if strings.Contains(line, "|") {
			pages := strings.Split(line, "|")
			if beforeRule[pages[1]] == nil {
				beforeRule[pages[1]] = make(map[string]struct{})
			}
			beforeRule[pages[1]][pages[0]] = struct{}{}
			if afterRule[pages[0]] == nil {
				afterRule[pages[0]] = make(map[string]struct{})
			}
			afterRule[pages[0]][pages[1]] = struct{}{}
		}
		if strings.Contains(line, ",") {
			pages := strings.Split(line, ",")
			if slices.IsSortedFunc(pages, sort) {
				a, _ := strconv.Atoi(pages[len(pages)/2])
				ans += a
			}
		}
	}

	// fmt.Println(beforeRule)
	// fmt.Println(afterRule)

	return strconv.FormatInt(int64(ans), 10)
}

func SolutionDay05part2(input []string) string {
	ans := 0
	beforeRule := make(map[string]map[string]struct{})
	afterRule := make(map[string]map[string]struct{})

	sort := func(a, b string) int {
		if _, ok := beforeRule[a][b]; ok {
			return 1
		}
		if _, ok := afterRule[a][b]; ok {
			return -1
		}
		return 0
	}

	for _, line := range input {
		if strings.Contains(line, "|") {
			pages := strings.Split(line, "|")
			if beforeRule[pages[1]] == nil {
				beforeRule[pages[1]] = make(map[string]struct{})
			}
			beforeRule[pages[1]][pages[0]] = struct{}{}
			if afterRule[pages[0]] == nil {
				afterRule[pages[0]] = make(map[string]struct{})
			}
			afterRule[pages[0]][pages[1]] = struct{}{}
		}
		if strings.Contains(line, ",") {
			pages := strings.Split(line, ",")
			if !slices.IsSortedFunc(pages, sort) {
				slices.SortFunc(pages, sort)
				a, _ := strconv.Atoi(pages[len(pages)/2])
				ans += a
			}
		}
	}

	// fmt.Println(beforeRule)
	// fmt.Println(afterRule)

	return strconv.FormatInt(int64(ans), 10)
}
