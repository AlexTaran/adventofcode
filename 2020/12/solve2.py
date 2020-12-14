#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    cx = 0
    cy = 0
    wx = 10
    wy = 1
    def mv(dr, vl):
      nonlocal wx
      nonlocal wy
      if dr == 'N': wy += vl
      if dr == 'S': wy -= vl
      if dr == 'E': wx += vl
      if dr == 'W': wx -= vl
    for l in lines:
      val = int(l[1:])
      if l[0] in 'LR':
        v = val // 90
        if l[0] == 'R': v = 4 - v
        for i in range(v):
          wx, wy = -wy, wx
      elif l[0] == 'F':
        cx += wx * val
        cy += wy * val
      else:
        mv(l[0], val)
    print(abs(cx)+abs(cy))

if __name__=="__main__":
    main()
