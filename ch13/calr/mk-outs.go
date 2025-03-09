package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"sync"
)

const OUTDIR = "tests/expected"

var fileDelete = new(sync.WaitGroup)

func main() {
	_, err := os.Stat(OUTDIR)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			if err = os.MkdirAll(OUTDIR, 0755); err != nil {
				log.Fatal(err)
			}
		} else {
			log.Fatal(err)
		}
	}

	OUTDIRFiles, err := filepath.Glob(OUTDIR + "/*")
	if err != nil {
		log.Fatal(err)
	}
	for _, f := range OUTDIRFiles {
		if err = os.RemoveAll(f); err != nil {
			log.Fatal(err)
		}
	}

	run_ccal(filepath.Join(OUTDIR, "2020.txt"), "2020")
	run_ccal(filepath.Join(OUTDIR, "2-2020.txt"), "2", "2020")
	run_ccal(filepath.Join(OUTDIR, "4-2020.txt"), "4", "2020")
	run_ccal(filepath.Join(OUTDIR, "5-2020.txt"), "5", "2020")
	fileDelete.Wait()
}

func run_ccal(outfile string, args ...string) {
	cmd := exec.Command("ccal", args...)
	_, err := os.Stat(outfile)
	runEnd := make(chan struct{})
	if errors.Is(err, os.ErrNotExist) {
		f, err := os.Create(outfile)
		if err != nil {
			log.Fatal(err)
		}
		defer func(end chan struct{}) {
			f.Close()
			close(runEnd)
		}(runEnd)
		cmd.Stdout = f
	} else {
		log.Fatal("Cannot create file:", outfile)
		os.Exit(1)
	}
	fmt.Println(cmd.String())

	if err := cmd.Run(); err != nil {
		fileDelete.Add(1)
		go func(filename string, c chan struct{}) {
			<-c
			os.Remove(filename)
			fileDelete.Done()
		}(outfile, runEnd)
		log.Fatal(err)
	}
}
