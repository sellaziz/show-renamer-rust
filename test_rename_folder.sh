#!/bin/sh

source set_api_key.sh

mkdir testcase
testnames=("the office 01.mp4" "The office Season 2 episode 4.mp4" "Breaking bad episode 5 season 2.avi")
for i in ${!testnames[@]};
do
    testname=${testnames[$i]}
    touch "testcase/$testname"
done

env TMDB_API_KEY=$MY_TMDB_API_KEY cargo run -- -d ./testcase/

rm -rf testcase
