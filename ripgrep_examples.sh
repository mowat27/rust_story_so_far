#!/usr/bin/env bash


cd ~/code/mowat27/play/linux || exit 1

echo "INFO : running UNIX find"
time (find . -path .git -prune -o -type f -exec grep '[A-Z]+_SUSPEND' {} \; | wc -l)

echo "INFO : running git grep"
time (git grep -P -n -w '[A-Z]+_SUSPEND' | wc -l)

echo "INFO : running silver surfer (ag)"
time (ag -w '[A-Z]+_SUSPEND' | wc -l)

echo "INFO : running ripgrep"
time (rg -n -w '[A-Z]+_SUSPEND' | wc -l)


# $ ./ripgrep_examples.sh
# INFO : running UNIX find
#        0

# real	6m31.126s
# user	4m9.999s
# sys	1m50.665s
# INFO : running git grep
#      462

# real	0m2.526s
# user	0m3.367s
# sys	0m5.153s
# INFO : running silver surfer (ag)
#      462

# real	0m2.858s
# user	0m5.534s
# sys	0m4.543s
# INFO : running ripgrep
#      462

# real	0m0.609s
# user	0m1.278s
# sys	0m3.048s
