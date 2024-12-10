package solutions

import (
	"fmt"
	"strconv"
)

func SolutionDay09part1(input []string) string {
	ans := 0
	len := len(input[0])
	leftCurrent := 0
	stack := make([]int, 0)
	right := -1
	foundLeft := false
	for i := 0; i < len; i++ {
		if i%2 == 0 {
			count, _ := strconv.Atoi(string(input[0][i]))
			for j := 0; j < count; j++ {
				stack = append(stack, leftCurrent)
			}
			leftCurrent += 1
		} else {
			count, _ := strconv.Atoi(string(input[0][i]))
			for j := 0; j < count; j++ {
				stack = append(stack, -1)
			}
		}
	}

	for i, val := range stack {
		if val == -1 && !foundLeft {
			foundLeft = true
		}
		if val != -1 {
			right = i
		}
	}

	ans = checkSum(stack, right)
	return strconv.FormatInt(int64(ans), 10)
}

func checkSum(stack []int, right int) int {
	ans := 0
	for i := 0; i < len(stack); i++ {
		if right < i {
			return ans
		}
		if stack[i] != -1 {
			ans += stack[i] * i
		} else {
			for stack[right] == -1 {
				right -= 1
			}
			if right < i {
				return ans
			} else {
				ans += stack[right] * i
				right -= 1
			}
		}
	}
	return ans
}

type file struct {
	id         int
	startIndex int
	size       int
}

func SolutionDay09part2(input []string) string {
	ans := int64(0)
	length := len(input[0])
	files := make([]file, 0)
	disk := make([]int, 0)
	leftCurrent := 0
	for i := 0; i < length; i++ {
		count, _ := strconv.Atoi(string(input[0][i]))
		if i%2 == 0 {
			startIndex := len(disk)
			for j := 0; j < count; j++ {
				disk = append(disk, leftCurrent)
			}
			files = append(files, file{startIndex: startIndex, size: count, id: leftCurrent})
			leftCurrent += 1
		} else {
			for j := 0; j < count; j++ {
				disk = append(disk, -1)
			}
		}
	}
	fmt.Println(files)
	for i := len(files) - 1; i >= 0; i-- {
		defrag(i, files[i], disk)
	}
	ans = checksum(disk)
	return strconv.FormatInt(ans, 10)
}

func defrag(fileNumber int, file file, numbers []int) {
	pointer := 0
	for pointer < file.startIndex {
		for pointer < file.startIndex && numbers[pointer] != -1 {
			pointer++
		}

		if pointer == file.startIndex {
			break
		}

		cntDots := 0
		startDots := pointer
		for pointer < file.startIndex && numbers[pointer] == -1 {
			cntDots++
			pointer++
		}

		if cntDots >= file.size {
			moveFile(fileNumber, file, numbers, startDots)
			break
		}
	}
}

func moveFile(fileNumber int, file file, numbers []int, startDots int) {
	for i := 0; i < file.size; i++ {
		numbers[startDots+i] = fileNumber
		numbers[file.startIndex+i] = -1
	}
}

func checksum(numbers []int) int64 {
	sum := int64(0)
	for i, num := range numbers {
		if num == -1 {
			continue
		}

		sum += int64(num) * int64(i)
	}
	return sum
}
