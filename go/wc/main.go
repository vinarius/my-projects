package main

import (
	"bytes"
	"flag"
	"fmt"
	"io"
	"os"
	"strings"
	"unicode/utf8"
)

func checkError(err error) {
	if err != nil {
		panic(err)
	}
}

var p = fmt.Println

func main() {
	printByteCount := flag.Bool("c", false, "print the byte count")
	printCharacterCount := flag.Bool("m", false, "print the character count")
	printNewlineCount := flag.Bool("l", false, "print the newline count")
	printWordCount := flag.Bool("w", false, "print the word count")

	flag.Parse()

	args := flag.Args()
	var contentsInBytes []byte
	var fileLocation string

	if len(args) > 0 {
		fileLocation = args[0]

		data, err := os.ReadFile(fileLocation)
		checkError(err)

		contentsInBytes = data
	} else {
		data, err := io.ReadAll(os.Stdin)
		checkError(err)

		contentsInBytes = data
	}

	if *printByteCount {
		byteCount := getByteCount(&contentsInBytes)

		p(byteCount, fileLocation)

		return
	}

	if *printNewlineCount {
		lineCount := getNewlineCount(&contentsInBytes)

		p(lineCount, fileLocation)

		return
	}

	if *printWordCount {
		wordCount := getWordcount(&contentsInBytes)

		p(wordCount, fileLocation)

		return
	}

	if *printCharacterCount {
		characterCount := utf8.RuneCount(contentsInBytes)

		p(characterCount, fileLocation)

		return
	}

	byteCount := getByteCount(&contentsInBytes)
	lineCount := getNewlineCount(&contentsInBytes)
	wordCount := getWordcount(&contentsInBytes)

	p(lineCount, wordCount, byteCount, fileLocation)

}

func getByteCount(dataInBytes *[]byte) int {
	return len(*dataInBytes)
}

func getNewlineCount(dataInBytes *[]byte) int {
	return bytes.Count(*dataInBytes, []byte{'\n'})
}

func getWordcount(dataInBytes *[]byte) int {
	fileContents := string(*dataInBytes)
	return len(strings.Fields(fileContents))

}
