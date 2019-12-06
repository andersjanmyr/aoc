package main

import (
	"fmt"
	"strconv"
)

func main() {
	pwds := calcPasswords(235741, 706948)
	fmt.Println(pwds)
	fmt.Println(len(pwds))
}

func calcPasswords(min, max int) []int {
	pwds := []int{}
	for i := min; i <= max; i++ {
		if isValid(i) {
			pwds = append(pwds, i)
		}
	}
	return pwds
}

func isValid(i int) bool {
	is := digits(i)
	return hasDouble(is) && increasing(is)
}

func hasDouble(is []int) bool {
	streak := 1
	for i := 0; i < len(is)-1; i++ {
		if is[i] == is[i+1] {
			streak++
		} else {
			if streak == 2 {
				return true
			}
			streak = 1
		}
	}
	return streak == 2
}

func increasing(is []int) bool {
	for i := 0; i < len(is)-1; i++ {
		if is[i] > is[i+1] {
			return false
		}
	}
	return true
}

func digits(i int) []int {
	is := []int{}
	s := strconv.Itoa(i)
	for i := 0; i < len(s); i++ {
		i, _ := strconv.Atoi(s[i : i+1])
		is = append(is, i)
	}
	return is
}
