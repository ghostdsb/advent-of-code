package solutions

import (
	"fmt"
	"strconv"
	"strings"
)

func SolutionDay22(input []string) string {

	secretNumbers := make([]int, 0)
	for _, line := range input {
		secret, _ := strconv.Atoi(line)
		secretNumbers = append(secretNumbers, secret)
	}
	part1 := d22p1(secretNumbers)
	part2 := d22p2(secretNumbers)

	return fmt.Sprintf("%s %s", part1, part2)
}

func d22p1(input []int) string {
	ans := 0
	for _, secret := range input {
		for i := 0; i < 2000; i++ {
			secret = processSecret(secret)
		}
		ans += secret
		// fmt.Println(secret)
	}
	return strconv.FormatInt(int64(ans), 10)
}

type deltaSeq struct {
	value int
	count int
}

func d22p2(input []int) string {
	ans := 0
	prevValue := 0
	sequenceTrendSumMap := make(map[string]int)
	for _, secret := range input {
		sequence := make([]int, 0)
		sequenceMap := make(map[string]deltaSeq)
		for i := 0; i < 2000; i++ {
			secret = processSecret(secret)
			value := secret % 10
			delta := value - prevValue

			sequence = append(sequence, delta)

			if i > 3 {
				sequence = sequence[1:]
			}

			if i >= 3 {
				sequenceStr := make([]string, len(sequence))
				for i, num := range sequence {
					sequenceStr[i] = strconv.Itoa(num)
				}
				if item, present := sequenceMap[strings.Join(sequenceStr, ",")]; present {
					sequenceMap[strings.Join(sequenceStr, ",")] = deltaSeq{item.value, item.count + 1}
				} else {
					sequenceMap[strings.Join(sequenceStr, ",")] = deltaSeq{value, 1}
				}
			}

			prevValue = value
		}
		for k, v := range sequenceMap {
			sequenceTrendSumMap[k] += v.value
		}
		// fmt.Println(findMaxKey(sequenceTrendSumMap))
		_, ans = findMaxKey(sequenceTrendSumMap)
		// fmt.Println(trends)
	}
	return strconv.FormatInt(int64(ans), 10)
}

func findMaxKey(m map[string]int) (string, int) {
	if len(m) == 0 {
		return "", 0
	}

	var maxKey string
	var maxValue int
	first := true

	for k, v := range m {
		if first || v > maxValue {
			maxKey = k
			maxValue = v
			first = false
		}
	}

	return maxKey, maxValue
}

func processSecret(secret int) int {
	multiplyBy64 := secret * 64
	secret = mix(multiplyBy64, secret)
	secret = prune(secret)
	divideBy32 := secret / 32
	secret = mix(divideBy32, secret)
	secret = prune(secret)
	multiplyBy2048 := secret * 2048
	secret = mix(multiplyBy2048, secret)
	secret = prune(secret)
	return secret
}

func mix(input, secret int) int {
	return input ^ secret
}

func prune(secret int) int {
	return secret % 16777216
}
