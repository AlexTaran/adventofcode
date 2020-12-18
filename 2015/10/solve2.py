#!/usr/bin/env python3

from collections import defaultdict

def convert(l):
  res = ''
  i = 0
  while i < len(l):
    j = i
    while j < len(l) and l[i] == l[j]: j += 1
    res += str(j-i) + l[i]
    i = j
  return res

def main():
    line = [l.strip() for l in open('input.txt')][0]
    for i in range(50):
      line = convert(line)
    print(len(line))

if __name__=="__main__":
    main()
