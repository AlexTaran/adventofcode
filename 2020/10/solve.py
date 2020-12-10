#!/usr/bin/env python3

from collections import defaultdict

def main():
    nums = sorted([int(l.strip()) for l in open('input.txt')])
    nums = [0] + nums + [nums[-1] + 3]
    diffs = defaultdict(int)
    for i in range(len(nums) - 1):
      diffs[nums[i+1] - nums[i]] += 1
    print(diffs[1] * diffs[3])

if __name__=="__main__":
    main()
