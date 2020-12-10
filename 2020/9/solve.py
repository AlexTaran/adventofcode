#!/usr/bin/env python3

from collections import defaultdict

def main():
    nums = [int(l.strip()) for l in open('input.txt')]
    for i in range(25, len(nums)):
      slags = nums[i-25:i]
      found = False
      for j in range(len(slags)):
        for k in range(j + 1, len(slags)):
          if slags[j] + slags[k] == nums[i]:
            found = True
            break
      if not found:
        print(nums[i])
        return

if __name__=="__main__":
    main()
