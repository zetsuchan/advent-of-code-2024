# Day 1: Historian Hysteria

## Problem Summary
The Chief Historian has gone missing and the Elvish Senior Historians need help reconciling two lists of location IDs. The challenge involves:

1. Taking two lists of numbers (location IDs)
2. Pairing them up by their relative position when sorted
3. Calculating the total distance between paired numbers

### Example:
Given lists:
```
Left:  3,4,2,1,3,3
Right: 4,3,5,3,9,3
```

Process:
- Sort both lists independently
- Pair numbers by position
- Calculate absolute difference between pairs
- Sum all differences

## Part 1 Challenge
Calculate the total distance between two lists of location IDs by:
1. Sorting both lists independently
2. Pairing numbers by their sorted position
3. Finding the absolute difference between each pair
4. Summing all differences

## Implementation Notes

### Rust Implementation
Location: `/rust`
- Uses iterators and standard library sorting
- Focuses on memory efficiency and zero-cost abstractions

### Solidity Implementation
Location: `/solidity`
- Implements sorting on-chain
- Handles integer arithmetic carefully to prevent overflow

### Zig Implementation
Location: `/zig`
- Uses comptime features where applicable
- Focuses on performance and memory safety

## Testing
Each implementation includes test cases based on the example provided in the problem description.