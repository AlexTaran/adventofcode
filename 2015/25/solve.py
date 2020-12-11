#!/usr/bin/env python3

from collections import defaultdict

def nxt(num):
  return (num * 252533) % 33554393

def main():
    tr = 2947
    tc = 3029
    data = { (1, 1): 20151125 }
    cx = 1
    cy = 1
    while (tc, tr) not in data:
      z = data[(cx, cy)]
      if cy == 1:
        cx, cy = 1, cx + 1
      else:
        cx, cy = cx + 1, cy - 1
      data[(cx, cy)] = nxt(z)
    print(data[(tc, tr)])

if __name__=="__main__":
    main()
