#!/usr/bin/env bash


cd ~/code/mowat27/play/linux || exit 1

echo "INFO : standard grep"
time grep -E -r -n --include='*.c' --include='*.h' -w '[A-Z]+_SUSPEND' . | wc -l

echo "INFO : running The Silver Searcher (ag)"
time ag -w '[A-Z]+_SUSPEND' . | wc -l

echo "INFO : running ripgrep"
time rg -w '[A-Z]+_SUSPEND' . | wc -l
