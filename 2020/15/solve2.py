#!/usr/bin/env python3

def main():
    lines = [l.strip() for l in open('input.txt')]
    nums = list(map(int, lines[0].split(',')))
    lasts = {n:i for i, n in enumerate(nums)}
    del lasts[nums[-1]]
    for i in range(30000000 - len(nums)):
      k = nums[-1]
      if k not in lasts:
        nums.append(0)
      else:
        nums.append(len(nums) - lasts[k] - 1)
      lasts[nums[-2]] = len(nums) - 2
    print(nums[-1])

if __name__=="__main__":
    main()
