package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
)

var DIR string

func init() {
	flag.Usage = func() {
		fmt.Printf("Usage: %s DIR\n", os.Args[0])
		flag.PrintDefaults()
	}
}

func main() {
	flag.Parse()
	DIR = flag.Arg(0)
	if DIR == "" {
		flag.Usage()
		os.Exit(1)
	}

	err := os.Chmod(filepath.Join(DIR, "tests/inputs/dir"), 0755)
	if err != nil {
		log.Fatal(err)
	}
	err = os.Chmod(filepath.Join(DIR, "tests/inputs/fox.txt"), 0600)
	if err != nil {
		log.Fatal(err)
	}
	files := []string{
		"tests/inputs/.hidden", "tests/inputs/empty.txt", "tests/inputs/bustle.txt", "tests/inputs/dir/.gitkeep", "tests/inputs/dir/spiders.txt",
	}
	for _, file := range files {
		err = os.Chmod(filepath.Join(DIR, file), 0644)
		if err != nil {
			log.Fatal(err)
		}
	}

	fmt.Println("Done, fixed files in", DIR)
}
