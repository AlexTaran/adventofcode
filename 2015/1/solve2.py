#!/usr/bin/env python3

from collections import defaultdict

def main():
  res = 0
  line = [l.strip() for l in open('input.txt')][0]
  nums = [{'(': 1, ')': -1}[ch] for ch in line]
  acc = 0
  for i, n in enumerate(nums):
    acc += n
    if acc < 0:
      print(i+1)
      break

if __name__=="__main__":
    main()
