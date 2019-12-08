package main

import (
	"fmt"
	"strings"
)

func main() {
	input := lines()
	ss := decode(input[0], 25, 6)
	for _, s := range ss {
		fmt.Println(s)
	}
	fmt.Println(decode(input[0], 25, 6))
}

func decode(input string, w, h int) []string {
	ls := layers(input, w, h)
	image := []string{}
	for r := 0; r < h; r++ {
		s := ""
		for c := 0; c < w; c++ {
			p := ""
			for l, _ := range ls {
				p = ls[l][r][c : c+1]
				if p != "2" {
					break
				}
			}
			s += p
		}
		image = append(image, s)
	}
	return image
}

func onesTimesTwos(ss []string) int {
	return charCount(ss, "1") * charCount(ss, "2")
}
func minLayer(input string, w, h int) []string {
	ls := layers(input, w, h)
	_, i := minZeros(ls)
	return ls[i]
}

func layers(input string, w, h int) [][]string {
	ss := chunks(input, w*h)
	sss := [][]string{}
	for _, s := range ss {
		sss = append(sss, chunks(s, w))
	}
	return sss
}

func chunks(s string, l int) []string {
	ss := []string{}
	for len(s) > 0 {
		ss = append(ss, s[0:l])
		s = s[l:]
	}
	return ss
}

func minZeros(layers [][]string) (int, int) {
	min := 100000
	index := 10000
	for i, l := range layers {
		sum := charCount(l, "0")
		if sum < min {
			min = sum
			index = i
		}
	}
	return min, index
}

func charCount(layer []string, c string) int {
	sum := 0
	for _, row := range layer {
		c := strings.Count(row, c)
		sum += c
	}
	return sum
}
