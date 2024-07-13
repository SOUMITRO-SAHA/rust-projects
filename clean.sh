#!/bin/bash

#####################################################
# Author: Soumitra Saha
# Date: 11/09/2024
# Description: This script automatically cleans the project by removing all executable files for Java, C++, Rust, Go.
# Version: v1.1.0
#####################################################

set -e # Exit on error
set -x # Enable debug mode

# Find and delete all `application/x-pie-executable` files
find . -type f -exec file --mime-type {} + | grep 'application/x-pie-executable' | awk -F: '{print $1}' | xargs rm -f

echo "Finally: All executable files have been deleted."

# Exit with the status of the last command executed
exit $?
