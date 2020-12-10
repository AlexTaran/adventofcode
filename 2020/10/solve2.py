#!/usr/bin/env python3

from collections import defaultdict

def main():
    nums = sorted([int(l.strip()) for l in open('input.txt')])
    nums = [0] + nums + [nums[-1] + 3]
    dyn = [1]
    for i in range(1, len(nums)):
      res = 0
      for dlt in [1, 2, 3]:
        if i - dlt < 0: continue
        if nums[i] - nums[i-dlt] <= 3:
          res += dyn[i-dlt]
      dyn.append(res)
    print(dyn[-1])

if __name__=="__main__":
    main()
