# Purpose
A simple digital clock that prints to the terminal, used as a reference project for hardware endeavours.
- Note; I don't think using the epoch time as done here is efficient for most cases, it is definitely better to just sleep the thread for a second instead and achieve the same result.

## Building
Clone it, build using "cargo build" or "cargo run" and have your terminal filled with the time in 24 hour format.

## Dependencies
- chrono
- colored (change line 31 if you wish to remove it / not use it.)
