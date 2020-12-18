#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    for l in lines:
      res += len(l) - len(eval(l))
    print(res)

if __name__=="__main__":
    main()
