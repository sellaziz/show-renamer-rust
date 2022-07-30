#!/bin/sh

source set_api_key.sh

mkdir testcase
testname="the office ep 4 season 4.avi"
touch "testcase/$testname"

env TMDB_API_KEY=$MY_TMDB_API_KEY cargo run -- -f "testcase/$testname"

rm -rf testcase
