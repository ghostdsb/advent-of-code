package solutions

import (
	"fmt"
	"strconv"
)

type Coord struct {
	y, x int
}

type Keypad struct {
	currentPos Coord
	keypadType string
}

func (k *Keypad) findTargetPosition(targetKey rune) Coord {
	directionalMap := map[rune]Coord{
		'^': {0, 1},
		'<': {1, 0},
		'>': {1, 2},
		'v': {1, 1},
		'A': {0, 2},
	}
	numpadMap := map[rune]Coord{
		'0': {3, 1},
		'1': {2, 0},
		'2': {2, 1},
		'3': {2, 2},
		'4': {1, 0},
		'5': {1, 1},
		'6': {1, 2},
		'7': {0, 0},
		'8': {0, 1},
		'9': {0, 2},
		'A': {3, 2},
	}
	if k.keypadType == "directional" {
		return directionalMap[targetKey]
	} else {
		return numpadMap[targetKey]
	}
}

func (k *Keypad) getDirectionSequence(targetKey rune) (string, Coord) {
	directionalMap := map[rune]Coord{
		'^': {0, 1},
		'<': {1, 0},
		'>': {1, 2},
		'v': {1, 1},
		'A': {0, 2},
	}
	numpadMap := map[rune]Coord{
		'0': {3, 1},
		'1': {2, 0},
		'2': {2, 1},
		'3': {2, 2},
		'4': {1, 0},
		'5': {1, 1},
		'6': {1, 2},
		'7': {0, 0},
		'8': {0, 1},
		'9': {0, 2},
		'A': {3, 2},
	}
	sequence := ""
	var targetCoord Coord
	if k.keypadType == "directional" {
		targetCoord = directionalMap[targetKey]

	} else {
		targetCoord = numpadMap[targetKey]

	}
	if k.keypadType == "numerical" {
		if targetCoord.x > k.currentPos.x {
			for i := 0; i < targetCoord.x-k.currentPos.x; i++ {
				sequence += ">"
			}
			if targetCoord.y > k.currentPos.y {
				for i := 0; i < targetCoord.y-k.currentPos.y; i++ {
					sequence += "v"
				}
			} else {
				for i := 0; i < k.currentPos.y-targetCoord.y; i++ {
					sequence += "^"
				}
			}
		} else {
			if k.currentPos.y == 3 {
				for i := 0; i < k.currentPos.x-max(1, targetCoord.x); i++ {
					sequence += "<"
				}
				if targetCoord.y > k.currentPos.y {
					for i := 0; i < targetCoord.y-k.currentPos.y; i++ {
						sequence += "v"
					}
				} else {
					for i := 0; i < k.currentPos.y-targetCoord.y; i++ {
						sequence += "^"
					}
				}
				for i := 0; i < 1-targetCoord.x; i++ {
					sequence += "<"
				}
			} else {
				for i := 0; i < k.currentPos.x-targetCoord.x; i++ {
					sequence += "<"
				}
				if targetCoord.y > k.currentPos.y {
					for i := 0; i < targetCoord.y-k.currentPos.y; i++ {
						sequence += "v"
					}
				} else {
					for i := 0; i < k.currentPos.y-targetCoord.y; i++ {
						sequence += "^"
					}
				}

			}
		}

	} else {
		if targetCoord.x > k.currentPos.x {
			for i := 0; i < targetCoord.x-k.currentPos.x; i++ {
				sequence += ">"
			}
			if targetCoord.y > k.currentPos.y {
				for i := 0; i < targetCoord.y-k.currentPos.y; i++ {
					sequence += "v"
				}
			} else {
				for i := 0; i < k.currentPos.y-targetCoord.y; i++ {
					sequence += "^"
				}
			}
		} else {
			if targetCoord.y > k.currentPos.y {
				for i := 0; i < targetCoord.y-k.currentPos.y; i++ {
					sequence += "v"
				}
			} else {
				for i := 0; i < k.currentPos.y-targetCoord.y; i++ {
					sequence += "^"
				}
			}
			for i := 0; i < k.currentPos.x-targetCoord.x; i++ {
				sequence += "<"
			}
		}
	}

	sequence += "A"
	return sequence, targetCoord
}

func SolutionDay21(input []string) string {
	ans := 0
	codes := make([]string, 0)
	codes = append(codes, input...)
	botA := Keypad{Coord{3, 2}, "numerical"}
	botB := Keypad{Coord{0, 2}, "directional"}
	botC := Keypad{Coord{0, 2}, "directional"}
	for _, code := range codes {
		fullSequenceA := ""
		fullSequenceB := ""
		fullSequenceC := ""
		digits := ""
		for _, c := range code {
			sequence, finalCoord := botA.getDirectionSequence(c)
			botA.currentPos = finalCoord
			fullSequenceA += sequence
			_, err := strconv.Atoi(string(c))
			if err == nil {
				digits += string(c)
			}
		}
		fmt.Println(fullSequenceA)
		for _, c := range fullSequenceA {
			sequence, finalCoord := botB.getDirectionSequence(c)
			botB.currentPos = finalCoord
			fullSequenceB += sequence
		}
		for _, c := range fullSequenceB {
			sequence, finalCoord := botC.getDirectionSequence(c)
			botC.currentPos = finalCoord
			fullSequenceC += sequence
		}

		fmt.Println(fullSequenceA)
		fmt.Println(fullSequenceB)
		fmt.Println(fullSequenceC)
		numPart, _ := strconv.Atoi(digits)
		fmt.Printf("%d x %d\n", numPart, len(fullSequenceC))
		ans += numPart * len(fullSequenceC)
		botA.currentPos = Coord{3, 2}
		botB.currentPos = Coord{0, 2}
		botC.currentPos = Coord{0, 2}
		fmt.Println()

	}
	fmt.Println(ans)
	part1 := d21p1(input)
	part2 := d21p2(input)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d21p1(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}

func d21p2(input []string) string {
	ans := 0

	return strconv.FormatInt(int64(ans), 10)
}

// v<<A>>^AvA^Av<<A>>^AAv<A<A>>^AAvAA^<A>Av<A>^AA<A>Av<A<A>>^AAAvA^<A>A
// <v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A

// ^A<<^^A>>AvvvA
// ^A^^<<A>>AvvvA

// <A>A<AAv<AA>>^AvAA^Av<AAA>^A
// <A>Av<<AA>^AA>AvAA^A<vAAA>^A
