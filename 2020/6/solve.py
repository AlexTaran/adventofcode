#!/usr/bin/env python3

from collections import defaultdict

def handleData(lines):
  return len(set(''.join(lines)))

def main():
    lines = [l.strip() for l in open('input.txt')] + ['']
    data = []
    res = 0
    for l in lines:
      if len(l) == 0:
        res += handleData(data)
        data = []
        continue
      data.append(l)
    print(res)

if __name__=="__main__":
    main()
