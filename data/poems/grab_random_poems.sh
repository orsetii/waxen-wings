#!/bin/bash

# Check if the number of arguments is correct
if [ "$#" -ne 1 ]; then
	echo "Usage: $0 <number_of_poems>"
	exit 1
fi

# Store the number of poems to fetch
num_poems=$1

# Create an empty poems.json file if it doesn't exist
if [ ! -f poems.json ]; then
	echo "[]" >poems.json
fi

# Loop to fetch and append poems
for ((i = 1; i <= $num_poems; i++)); do
	# Fetch a random poem from the API
	poem=$(curl -s "https://poetrydb.org/random")

	# Parse the JSON response using jq
	parsed_poem=$(echo "$poem" | jq -c '.[]')

	# Read the existing poems from poems.json
	existing_poems=$(jq '.[]' poems.json)

	# Append the parsed poem to the existing poems
	new_poems=$(echo "$existing_poems" "$parsed_poem" | jq -s 'add')

	# Write the updated poems back to poems.json
	echo "$new_poems" >poems.json
done
