#!/usr/bin/env python3

from collections import defaultdict

def main():
    lines = [l.strip() for l in open('input.txt')]
    ids = []
    for l in lines:
      rw = int(''.join([{'F': '0', 'B': '1'}[ch]for ch in l[:7]]), 2)
      cl = int(''.join([{'R': '1', 'L': '0'}[ch]for ch in l[7:]]), 2)
      ids.append(rw * 8 + cl)
    ids = sorted(ids)
    for i in range(len(ids) - 1):
      if ids[i+1] - ids[i] == 2:
        print(ids[i] + 1)

if __name__=="__main__":
    main()
