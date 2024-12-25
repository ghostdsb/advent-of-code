package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

func SolutionDay19(input []string) string {

	stripes := make(map[string]bool)
	for _, stripe := range strings.Split(input[0], ", ") {
		stripes[stripe] = true
	}

	towels := make([]string, 0)
	towels = append(towels, input[2:]...)

	part1 := d19p1(stripes, towels)
	part2 := d19p2(stripes, towels)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d19p1(stripes map[string]bool, towels []string) string {
	ans := 0
	memo := make(map[string]bool)
	for _, towel := range towels {
		if canDesign(towel, stripes, memo) {
			ans += 1
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func d19p2(stripes map[string]bool, towels []string) string {
	ans := 0
	pool := make([]string, 0)
	for k := range stripes {
		pool = append(pool, k)
	}
	trie := build(pool)

	for _, w := range towels {
		if n := match(w, trie); n > 0 {
			// count1 += 1 // part1 answer
			ans += n
		}
	}
	return strconv.FormatInt(int64(ans), 10)
}

func canDesign(target string, stripes map[string]bool, memo map[string]bool) bool {
	if prefix, present := memo[target]; present {
		return prefix
	}
	if target == "" {
		return true
	}
	for pool := range stripes {
		if strings.HasPrefix(target, pool) {
			if canDesign(target[len(pool):], stripes, memo) {
				memo[target] = true
				return true
			}
		}
	}
	memo[target] = false
	return false
}

type TrieNode struct {
	next map[byte]*TrieNode
	stop bool
}

func newNode() *TrieNode {
	return &TrieNode{next: make(map[byte]*TrieNode)}
}

// Build a trie from a list of words
func build(words []string) *TrieNode {
	root := newNode()
	for _, word := range words {
		cur := root
		for _, x := range word {
			car := byte(x)
			if _, ok := cur.next[car]; !ok {
				cur.next[car] = newNode()
			}
			cur = cur.next[car]
		}
		cur.stop = true
	}
	return root
}

// Count all possible ways to fully match a string using words in the trie without overlaps
func match(line string, trie *TrieNode) int {
	end := len(line)
	memo := make(map[int]int, 58) // arbitrary size

	// DFS with memoization
	var recount func(int) int
	recount = func(start int) (count int) {
		if start == end {
			return 1 // success on the entire line!
		}

		if cnt, ok := memo[start]; ok {
			return cnt // use cached value
		}

		cur := trie
		for i := start; i < end; i++ {
			var ok bool
			var nxt *TrieNode

			car := line[i]
			if nxt, ok = cur.next[car]; !ok {
				break
			}

			cur = nxt
			if cur.stop {
				count += recount(i + 1) // add all ways from the next position
			}
		}

		memo[start] = count
		return
	}

	return recount(0)
}
