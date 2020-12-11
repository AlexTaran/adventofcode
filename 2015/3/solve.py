#!/usr/bin/env python3

from collections import defaultdict

def main():
  line = [l.strip() for l in open('input.txt')][0]
  cx = 0
  cy = 0
  ps = set([(cx, cy)])
  for ch in line:
    dx, dy = {'v': (0, -1), '^': (0, 1), '>': (1, 0), '<': (-1, 0)}[ch]
    cx += dx
    cy += dy
    ps.add((cx, cy))
  print(len(ps))

if __name__=="__main__":
    main()
