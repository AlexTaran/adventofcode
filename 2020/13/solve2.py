#!/usr/bin/env python3

from collections import defaultdict

def invr(x, m):
  for i in range(m):
    if (x * i) % m == 1:
      return i

def main():
    lines = [l.strip() for l in open('input.txt')]
    tms = [(int(s), i) for i, s in enumerate(lines[1].split(',')) if s != 'x']
    osts = [ (s - i) % s for s, i in tms]
    M = 1
    for s, i in tms: M *= s
    res = 0
    for i, ost in enumerate(osts):
      m = M // tms[i][0]
      im = invr(m % tms[i][0], tms[i][0])
      res += ost * m * im
    print(res % M)

if __name__=="__main__":
    main()
