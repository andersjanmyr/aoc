package main

import (
	"fmt"
	"io/ioutil"
)

func main() {
	bytes, _ := ioutil.ReadFile("input.txt")
	res := knotHash(string(bytes[0 : len(bytes)-1]))
	fmt.Println("knotHash", res)
}

func knotHash(s string) string {
	bytes := []byte(s)
	ns := make([]int, len(bytes))
	for i := 0; i < len(bytes); i++ {
		ns[i] = int(bytes[i])
	}
	return newHash(256, ns)
}

func newHash(n int, lengths []int) string {
	ns := nums(n)
	lengths = append(lengths, []int{17, 31, 73, 47, 23}...)
	newNs := reverseLengths(ns, lengths)
	return denseHash(newNs)
}

func denseHash(ns []int) string {
	items := make([]int, 16)
	for i := 0; i < 16; i++ {
		items[i] = denseHashItem(ns[i*16 : i*16+16])
	}
	s := ""
	for _, i := range items {
		s += fmt.Sprintf("%02x", i)

	}
	return s
}

func denseHashItem(ns []int) int {
	result := 0
	for i := 0; i < len(ns); i++ {
		result ^= ns[i]
	}
	return result
}

func hash(n int, lengths []int) int {
	ns := nums(n)
	newNs := reverseLengths(ns, lengths)
	return newNs[0] * newNs[1]
}

func nums(n int) []int {
	ns := make([]int, n)
	for i := 0; i < len(ns); i++ {
		ns[i] = i
	}
	return ns
}

func reverseLengths(ns, lengths []int) []int {
	pos := 0
	ln := len(ns)
	ll := len(lengths)
	for i := 0; i < ll*64; i++ {
		ii := i % ll
		l := lengths[ii]
		ns = reverse(ns, pos, l)
		pos = (pos + l + i) % ln
	}
	return ns
}

func reverse(ns []int, pos, length int) []int {
	nsLen := len(ns)
	results := make([]int, nsLen)
	copy(results, ns)
	ints := make([]int, length)
	for i := 0; i < length; i++ {
		ii := (pos + i) % nsLen
		ri := length - i - 1
		ints[ri] = ns[ii]
	}
	for i := 0; i < length; i++ {
		index := (pos + i) % nsLen
		results[index] = ints[i]
	}
	return results
}
