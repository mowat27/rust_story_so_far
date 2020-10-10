# minisearch

A simple search utility written in both Rust and Python for comparative purposes

A benchmark is included that searches for the work 'love' in the text of pride and prejudice.  Run `make benchmark` to execute it.

Results on my machine (2015 MacBook Pro, core i7) were...

```
Using fgrep...
time fgrep love examples/pride_and_prejudice.txt > /dev/null

real	0m0.022s
user	0m0.018s
sys	0m0.002s


Using Python...
time ./minisearch.py love examples/pride_and_prejudice.txt > /dev/null

real	0m0.102s
user	0m0.046s
sys	0m0.047s


Using Rust...
time target/release/minisearch love examples/pride_and_prejudice.txt > /dev/null

real	0m0.007s
user	0m0.004s
sys	0m0.002s
```