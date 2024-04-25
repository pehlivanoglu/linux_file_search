#!/bin/bash

# Check if the correct number of arguments is given
if [ $# -ne 2 ]; then
    echo "Usage: $0 <start_directory> <filename>"
    exit 1
fi

start_directory=$1
filename=$2

# Get the current time in milliseconds before starting the find
start_time=$(date +%s%3N)

# Execute the find command and stop after finding the first match
sudo find "$start_directory" -name "$filename" -print -quit

# Get the current time in milliseconds after the find command has finished
end_time=$(date +%s%3N)

# Calculate the elapsed time in milliseconds
elapsed_time=$((end_time - start_time))

echo "Time taken to find the file: $elapsed_time milliseconds"
