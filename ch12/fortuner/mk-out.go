// bash 스크립트보다 Go로 만든 프로그램이 낫지 않을까 싶어서 만든 프로그램
// 코드량은 크게 늘어나지만 차라리 이쪽이 나은 것 같다.
package main

import (
	"errors"
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"slices"
)

var InDir string
var OutDir string

func makeOutFile(outFileName string, args ...string) {

	cmd := exec.Command("fortune", args...)

	fStdout, err := os.Create(filepath.Join(OutDir, outFileName+".out"))
	if err != nil {
		log.Fatal(err)
	}
	defer fStdout.Close()
	fStderr, err := os.Create(filepath.Join(OutDir, outFileName+".err"))
	if err != nil {
		log.Fatal(err)
	}
	defer fStderr.Close()

	cmd.Stdout = fStdout
	cmd.Stderr = fStderr

	fmt.Println(cmd.Args)

	err = cmd.Start()
	if err != nil {
		log.Fatal(err)
	}
	err = cmd.Wait()
	if err != nil {
		log.Println(err)
	}
}

func main() {
	workingDir, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}

	InDir = filepath.Join(workingDir, "tests/inputs")
	OutDir = filepath.Join(workingDir, "tests/expected")

	info, err := os.Stat(OutDir)
	if errors.Is(err, os.ErrNotExist) {
		err = os.Mkdir(OutDir, 0755)
		if err != nil {
			log.Fatal(err)
		}
	} else if !info.IsDir() {
		err = fmt.Errorf("outDir is not a directory")
		log.Fatal(err)
	}

	outDirFiles, err := filepath.Glob(filepath.Join(OutDir, "*"))
	if err != nil {
		log.Fatal(err)
	}
	for _, outDirfile := range outDirFiles {
		if err = os.RemoveAll(outDirfile); err != nil {
			log.Fatal(err)
		}
	}

	files := []string{InDir + "/literature", InDir + "/quotes"}
	args := []string{"-m", "Yogi Berra"}
	args = slices.Concat(args, files)
	makeOutFile("berra_cap", args...)
	args = []string{"-m", "Mark Twain"}
	args = slices.Concat(args, files)
	makeOutFile("twain_cap", args...)

	args = []string{"-m", "yogi berra"}
	args = slices.Concat(args, files)
	makeOutFile("berra_lower", args...)
	args = []string{"-m", "mark twain"}
	args = slices.Concat(args, files)
	makeOutFile("twain_lower", args...)

	args = []string{"-i", "-m", "yogi berra"}
	args = slices.Concat(args, files)
	makeOutFile("berra_lower_i", args...)
	args = []string{"-i", "-m", "mark twain"}
	args = slices.Concat(args, files)
	makeOutFile("twain_lower_i", args...)
}
