#!/usr/bin/env python3

from collections import defaultdict
from ast import literal_eval

def inveval(l):
  return '"' + l.replace("\\", "\\\\").replace('"', '\\"') + '"'

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    for l in lines:
      res += len(inveval(l)) - len(l)
    print(res)

if __name__=="__main__":
    main()
