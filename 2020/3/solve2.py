#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 1
    lines = [l.strip() for l in open('input.txt')]
    slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
    for dx, dy in slopes:
      r = 0
      curx = 0
      cury = 0
      while cury < len(lines)-1:
        curx = (curx + dx) % len(lines[0])
        cury = (cury + dy) % len(lines)
        if lines[cury][curx] == '#':
          r += 1
      res *= r
    print(res)

if __name__=="__main__":
    main()
