#!/usr/bin/env python3

from collections import defaultdict

def main():
  line = [l.strip() for l in open('input.txt')][0]
  cx = 0
  cy = 0
  rx = 0
  ry = 0
  ps = set([(cx, cy)])
  for i, ch in enumerate(line):
    dx, dy = {'v': (0, -1), '^': (0, 1), '>': (1, 0), '<': (-1, 0)}[ch]
    if i % 2 == 0:
      cx += dx
      cy += dy
      ps.add((cx, cy))
    else: 
      rx += dx
      ry += dy
      ps.add((rx, ry))
  print(len(ps))

if __name__=="__main__":
    main()
