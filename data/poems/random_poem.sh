#!/bin/bash

# Check if poems.json file exists
if [ ! -f poems.json ]; then
	echo "Error: poems.json file not found."
	exit 1
fi

# Get the length of the root array
array_length=$(jq 'length' poems.json)

# Check if the array is empty
if [ "$array_length" -eq 0 ]; then
	echo "Error: poems.json file is empty."
	exit 1
fi

# Check if a maximum line count is provided
if [ "$#" -ne 1 ]; then
	echo "Usage: $0 <max_line_count>"
	exit 1
fi

max_lines="$1"

# Get all poems that meet the line count criteria
filtered_poems=$(cat poems.json | jq --argjson max_lines "$max_lines" '[.[] | select( .linecount | tonumber >= $max_lines)]')
poem_count=$(echo "$filtered_poems" | jq 'length')

# Check if any poems meet the criteria
if [ "$poem_count" -eq 0 ]; then
	echo "No poems found with line count less than or equal to $max_lines."
	exit 1
fi

# Generate a random index between 0 and (poem_count - 1)
random_index=$((RANDOM % poem_count))

# Get the random poem object that meets the criteria
random_poem=$(echo "$filtered_poems" | jq ".[$random_index]")

# Print the random poem object
echo "$random_poem"
