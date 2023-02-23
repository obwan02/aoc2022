#!/bin/bash

cargo init day${1} && curl "https://adventofcode.com/2022/day/${1}/input" -b session=53616c7465645f5f8684aaa7bcd05274cb5a57c0f67fd52dff73ed011e06edeefaa195049db0aa08e8e9af84f9c9a27187fc2318d1a862d367cb2d9c58b0afd8 -o day${1}/src/input.txt
