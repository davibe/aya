#!/bin/sh
# SUMMARY: Check that aya-gen works
# LABELS:

set -ex

# Source libraries. Uncomment if needed/defined
#. "${RT_LIB}"
# . "${RT_PROJECT_ROOT}/_lib/lib.sh"


# Test code goes here

echo "AAAAAAAAAAAAAAAAAAA"
find /__w/aya
aya-gen generate tcphdr
find /usr/src -name tcp.h