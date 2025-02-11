#!/usr/bin/env bash

set -u

ROOT="tests/inputs"
OUT_DIR="tests/expected"

[[ ! -d "$OUT_DIR" ]] && mkdir -p "$OUT_DIR"

EMPTY="$ROOT/empty.txt"
FOX="$ROOT/fox.txt"
SPIDERS="$ROOT/spiders.txt"
BUSTLE="$ROOT/the-bustle.txt"
ALL="$EMPTY $FOX $SPIDERS $BUSTLE"

for FILE in $ALL; do
    BASENAME=$(basename "$FILE")
    cat "$FILE" >$OUT_DIR/"${BASENAME}".out
    cat -n "$FILE" >$OUT_DIR/"${BASENAME}".n.out
    cat -b "$FILE" >$OUT_DIR/"${BASENAME}".b.out
done

# shellcheck disable=SC2086
cat $ALL >$OUT_DIR/all.out
# shellcheck disable=SC2086
cat -n $ALL >$OUT_DIR/all.n.out
# shellcheck disable=SC2086
cat -b $ALL >$OUT_DIR/all.b.out

# shellcheck disable=SC2094
cat <$BUSTLE >$OUT_DIR/"$(basename $BUSTLE)".stdin.out
# shellcheck disable=SC2094
cat -n <$BUSTLE >$OUT_DIR/"$(basename $BUSTLE)".n.stdin.out
# shellcheck disable=SC2094
cat -b <$BUSTLE >$OUT_DIR/"$(basename $BUSTLE)".b.stdin.out
