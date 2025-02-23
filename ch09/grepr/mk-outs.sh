#!/usr/bin/env bash

set -u

DIR="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"
rm -f "$OUT_DIR"/*

# 빈 파일
grep foo $DIR/empty.txt > "$OUT_DIR/foo.empty.txt"

# 빈 정규표현식
grep '' $DIR/fox.txt > "$OUT_DIR/empty_regex.fox.txt"

# 대소문자 구분
grep The $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.capitalized"
grep the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase"
grep -i the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.insensitive"
grep nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt"
grep -i nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.insensitive"

# 대소문자 구분, 여러 파일 입력
grep The $DIR/*.txt > "$OUT_DIR/all.the.capitalized"
grep -i the $DIR/*.txt > "$OUT_DIR/all.the.lowercase.insensitive"

# 재귀적으로 디렉터리 검색
grep -r dog $DIR > "$OUT_DIR/dog.recursive"

# 재귀적, 대소문자 무시
grep -ri then $DIR > "$OUT_DIR/the.recursive.insensitive"

# 대소문자 구분, 개수 표시
grep -c The $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.capitalized.count"
grep -c the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.count"
grep -ci the $DIR/bustle.txt > "$OUT_DIR/bustle.txt.the.lowercase.insensitive.count"
grep -c nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.count"
grep -ci nobody $DIR/nobody.txt > "$OUT_DIR/nobody.txt.insensitive.count"

# 대소문자 구분, 개수 표시, 여러 파일 입력
grep -c The $DIR/*.txt > "$OUT_DIR/all.the.capitalized.count"
grep -ci the $DIR/*.txt > "$OUT_DIR/all.the.lowercase.insensitive.count"

# 재귀적 검색, 대소문자 무시, 갯수 표시
grep -cri the $DIR > "$OUT_DIR/the.recursive.insensitive.count"

# STDIN, 대소문자 무시, 갯수 표시
cat $DIR/*.txt | grep -ci the - > "$OUT_DIR/the.recursive.insensitive.count.stdin"
