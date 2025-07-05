#!/bin/bash

> times.txt

sizes=(500000 1000000 5000000 10000000)

for size in "${sizes[@]}"; do
  for threads in {1..8}; do
    TIME=$(/usr/bin/time -f "%E" ./main $size $threads 2>&1 >/dev/null)
    echo "$size | $threads | $TIME" >> times.txt
  done
done

./graphs.py