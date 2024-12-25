package solutions

import (
	"fmt"
	"sort"
	"strconv"
	"strings"
)

type computer struct {
	name   string
	depth  int
	parent *computer
}

var count int = 0

func SolutionDay23(input []string) string {

	network := make(map[string][]string)
	first := ""
	for i, line := range input {
		comps := strings.Split(line, "-")
		if i == 0 {
			first = comps[0]
		}
		network[comps[0]] = append(network[comps[0]], comps[1])
		network[comps[1]] = append(network[comps[1]], comps[0])
	}

	part1 := d23p1(network, first)
	part2 := d23p2(network)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d23p1(network map[string][]string, first string) string {
	ans := 0
	visited := make(map[string]bool)
	visitedChild := make(map[string]computer)
	q := make([]computer, 0)
	q = append(q, computer{name: first, depth: 0, parent: nil})

	triplets := make(map[string]bool)
	for len(q) > 0 {
		top := q[0]
		q = q[1:]
		visited[top.name] = true
		visitedChild[top.name] = top
		for _, child := range network[top.name] {
			if _, ok := visited[child]; !ok {
				q = append(q, computer{name: child, depth: top.depth + 1, parent: &top})
			} else {
				if child != top.name && visitedChild[child].depth == top.depth && visitedChild[child].parent == top.parent {
					if child[0] == 't' || top.name[0] == 't' || top.parent.name[0] == 't' {
						tripletNameSlice := sort.StringSlice{child, top.name, top.parent.name}
						tripletNameSlice.Sort()
						tripletName := fmt.Sprintf("%s-%s-%s", tripletNameSlice[0], tripletNameSlice[1], tripletNameSlice[2])
						triplets[tripletName] = true
					}
				}
			}
		}
	}

	ans = len(triplets)
	return strconv.FormatInt(int64(ans), 10)
}

func d23p2(network map[string][]string) string {
	ans := 0
	for node, neighbours := range network {
		fmt.Println(node)

		conn := areInterconnected(neighbours, network)
		if conn {
			fmt.Println(node, neighbours, count)
		}
		if count > ans {
			ans = count
		}
		count = 0
	}
	return strconv.FormatInt(int64(ans+2), 10)
}

func areInterconnected(children []string, network map[string][]string) bool {
	count += 1
	if len(children) == 3 {
		a := network[children[0]]
		b := network[children[1]]
		c := network[children[2]]
		if contains(a, children[1]) && contains(a, children[2]) &&
			contains(b, children[0]) && contains(b, children[2]) &&
			contains(c, children[0]) && contains(c, children[1]) {
			return true
		} else {
			return false
		}
	}
	return areInterconnected(children[1:], network)
}

func contains(a []string, s string) bool {
	for _, x := range a {
		if x == s {
			return true
		}
	}
	return false
}
