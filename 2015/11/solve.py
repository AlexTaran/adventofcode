#!/usr/bin/env python3

from collections import defaultdict

def nxt(s):
  nums = [ord(ch) - ord('a') for ch in s]
  nums[-1] += 1
  for i in reversed(range(len(s))):
    if nums[i] == 26:
      nums[i] = 0
      nums[i-1] += 1
    else:
      break
  return ''.join([chr(n + ord('a')) for n in nums])

def isgood(s):
  if 'i' in s or 'o' in s or 'l' in s: return False
  numpairs = sum([1 if chr(i + ord('a')) * 2 in s else 0 for i in range(26)])
  if numpairs < 2: return False
  for i in range(2, len(s)):
    if ord(s[i-2]) + 1 == ord(s[i-1]) and ord(s[i-1]) + 1 == ord(s[i]):
      return True
  return False

def main():
    line = [l.strip() for l in open('input.txt')][0]
    while not isgood(line):
      line = nxt(line)
    print(line)

if __name__=="__main__":
    main()
