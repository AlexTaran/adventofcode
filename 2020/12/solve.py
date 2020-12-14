#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    dirs = ['E', 'N', 'W', 'S']
    cx = 0
    cy = 0
    d = 'E'
    def mv(dr, vl):
      nonlocal cx
      nonlocal cy
      if dr == 'N': cy += vl
      if dr == 'S': cy -= vl
      if dr == 'E': cx += vl
      if dr == 'W': cx -= vl
    for l in lines:
      val = int(l[1:])
      if l[0] in 'LR':
        v = val // 90
        if l[0] == 'R': v = 4 - v
        d = dirs[(dirs.index(d) + v) % 4]
      elif l[0] == 'F':
        mv(d, val)
      else:
        mv(l[0], val)
    print(abs(cx)+abs(cy))

if __name__=="__main__":
    main()
