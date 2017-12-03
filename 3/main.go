package main

import (
	"fmt"
)

/*
17  16  15  14  13
18   5   4   3  12
19   6   1   2  11
20   7   8   9  10
21  22  23---> ...
*/
func manhattan(n int) int {
	if n == 1 {
		return 0
	}
	pow := 8
	count := 1
	var i int
	for i = 1; count+pow < n; i++ {
		count += pow
		pow += 8
	}

	return i + delta(i, n-count)
}

func delta(level int, n int) int {
	v := level - 1
	d := -1
	if v == 0 {
		d = 1
	}
	for n > 1 {
		n--
		v += d
		if v == level || v == 0 {
			d = -d
		}
	}
	return v
}

func main() {
	fmt.Println(manhattan(1024))
	fmt.Println(manhattan(347991))
}
