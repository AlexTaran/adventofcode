#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    for l in lines:
      x,y,z = map(int, l.split('x'))
      a, b = sorted([x,y,z])[:2]
      res += 2*(x*y+y*z+x*z) + a*b
    print(res)

if __name__=="__main__":
    main()
