#!/bin/bash

# Change to the script's directory
cd "$(dirname "$0")"

# Get the list of .cql files and check if any exists
cql_files=$(ls *.cql 2>/dev/null)
if [ -z "$cql_files" ]; then
    echo "No .cql files found for migration."
    exit 0
fi

# Loop through each .cql file and run it with cqlsh
for file in $cql_files; do
    echo "Running $file"
    docker-compose exec scylla cqlsh --file "/migrations/$file"
done
