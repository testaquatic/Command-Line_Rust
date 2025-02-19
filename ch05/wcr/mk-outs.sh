#!/usr/bin/env bash

ROOT="tests/inputs"
FILES="$ROOT/empty.txt $ROOT/fox.txt $ROOT/atlamal.txt"
OUTDIR="tests/expected"

[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

for FILE in $FILES; do
    BASENAME=$(basename "$FILE")
    wc "$FILE" >${OUTDIR}/"${BASENAME}".out
    wc -l "$FILE" >${OUTDIR}/"${BASENAME}".l.out
    wc -w "$FILE" >${OUTDIR}/"${BASENAME}".w.out
    wc -c "$FILE" >${OUTDIR}/"${BASENAME}".c.out
    wc -m "$FILE" >${OUTDIR}/"${BASENAME}".m.out
    wc -lwm "$FILE" >${OUTDIR}/"${BASENAME}".lwm.out
    wc -wc "$FILE" >${OUTDIR}/"${BASENAME}".wc.out
    wc -wm "$FILE" >${OUTDIR}/"${BASENAME}".wm.out
    wc -wl "$FILE" >${OUTDIR}/"${BASENAME}".wl.out
    wc -cl "$FILE" >${OUTDIR}/"${BASENAME}".cl.out
    wc -ml "$FILE" >${OUTDIR}/"${BASENAME}".ml.out
done

wc <"$ROOT/atlamal.txt" >"$OUTDIR/atlamal.txt.stdin.out"

wc $FILES >"$OUTDIR/all.out"
wc -l $FILES >"$OUTDIR/all.l.out"
wc -w $FILES >"$OUTDIR/all.w.out"
wc -c $FILES >"$OUTDIR/all.c.out"
wc -m $FILES >"$OUTDIR/all.m.out"
wc -lwm $FILES >"$OUTDIR/all.lwm.out"
wc -wc $FILES >"$OUTDIR/all.wc.out"
wc -wm $FILES >"$OUTDIR/all.wm.out"
wc -wl $FILES >"$OUTDIR/all.wl.out"
wc -cl $FILES >"$OUTDIR/all.cl.out"
wc -ml $FILES >"$OUTDIR/all.ml.out"
