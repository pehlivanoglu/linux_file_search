#!/bin/bash

# Check if the correct number of arguments is given
if [ $# -ne 1 ]; then
    echo "Usage: $0 <filename>"
    exit 1
fi

filename=$1

# Get the current time in milliseconds before starting the locate
start_time=$(date +%s%3N)

# Execute the locate command
locate_results=$(locate -b "\\$filename")

# Display the results
echo "$locate_results"

# Get the current time in milliseconds after the locate command has finished
end_time=$(date +%s%3N)

# Calculate the elapsed time in milliseconds
elapsed_time=$((end_time - start_time))

echo "Time taken to locate the file: $elapsed_time milliseconds"
