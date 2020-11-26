# specialty
A simple utility for calculating the probability of achieving at least a given number of successes on a dice roll in the 20th Anniversary Storyteller (World of Darkness) system. It calculates probabilities for rolls with and without specialties.

## Usage
`./specialty <pool> <difficulty> <target>`

## Issues
Save to make sure they're numbers, `specialty` does no error checking on its arguments. Thus, it will crash spectacularly if the pool is negative, has undefined behavior if the difficulty is negative, and will give useless results if the target is negative.
