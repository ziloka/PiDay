#!/bin/bash

# $1, $2 means positional arguments
# this script should be used as the following
# ./time_run.sh 20000 1
# ./time_run.sh 20000 2
# ./time_run.sh 20000 4
# ./time_run.sh 20000 8
# ./time_run.sh 20000 6

for turn in $(seq 1 $3)
do
  result=$(/usr/bin/time -f "%E" ./target/release/multithreaded_pi_rust -e $1 -t $2 2>&1 > /dev/null);
  echo $result
done