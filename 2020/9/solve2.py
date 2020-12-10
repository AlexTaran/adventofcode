#!/usr/bin/env python3

from collections import defaultdict

def findInvalid(nums):
  for i in range(25, len(nums)):
    slags = nums[i-25:i]
    found = False
    for j in range(len(slags)):
      for k in range(j + 1, len(slags)):
        if slags[j] + slags[k] == nums[i]:
          found = True
          break
    if not found:
      return nums[i]

def main():
    nums = [int(l.strip()) for l in open('input.txt')]
    inval = findInvalid(nums)
    for fst in range(len(nums)):
      sm = nums[fst]
      for lst in range(fst+1, len(nums)):
         sm += nums[lst]
         if sm == inval:
            rng = nums[fst:lst+1]
            print(min(rng)+max(rng))
    

if __name__=="__main__":
    main()
