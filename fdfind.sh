#!/bin/bash


filename=$1

# Get the current time in milliseconds before starting the fdfind
start_time=$(date +%s%3N)

# Execute the fdfind command and stop after finding the first match
fdfind "$filename" --max-results 1

# Get the current time in milliseconds after the fdfind command has finished
end_time=$(date +%s%3N)

# Calculate the elapsed time in milliseconds
elapsed_time=$((end_time - start_time))

echo "Time taken to find the file: $elapsed_time milliseconds"
