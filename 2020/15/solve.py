#!/usr/bin/env python3

def main():
    lines = [l.strip() for l in open('input.txt')]
    nums = list(map(int, lines[0].split(',')))
    for i in range(2020 - len(nums)):
      k = nums[-1]
      j = len(nums) - 2
      while j >= 0 and nums[j] != k: j -= 1
      if j < 0:
        nums.append(0)
      else:
        nums.append(len(nums) - j - 1)
    print(nums[-1])

if __name__=="__main__":
    main()
