package main

import (
	"fmt"
	"io/ioutil"
	"sort"
	"strings"
)

func main() {
	phrases := lines()
	vc := 0
	for _, p := range phrases {
		if validPassphrase(p) {
			vc++
		}
	}
	fmt.Printf("Valid phrases: %d\n", vc)
}

func lines() []string {
	bytes, _ := ioutil.ReadFile("input.txt")
	data := string(bytes)
	return strings.Split(data, "\n")
}

/*
aa bb cc dd ee is valid.
aa bb cc dd aa is not valid - the word aa appears more than once.
aa bb cc dd aaa is valid
abcde fghij is a valid passphrase
abcde xyz ecdab is not valid
*/
func validPassphrase(phrase string) bool {
	ws := sortedWords(phrase)
	if len(ws) < 2 {
		return false
	}
	for i := 0; i < len(ws)-1; i++ {
		for j := i + 1; j < len(ws); j++ {
			if ws[i] == ws[j] {
				return false
			}
		}
	}
	fmt.Println(ws)
	return true
}

func words(line string) []string {
	return strings.Split(line, " ")
}

func sortedWords(line string) []string {
	ws := words(line)
	sws := make([]string, len(ws))
	for i, _ := range ws {
		sws[i] = sortWord(ws[i])
	}
	return sws
}

func sortWord(word string) string {
	chars := strings.Split(word, "")
	sort.Strings(chars)
	return strings.Join(chars, "")
}
