#!/usr/bin/env python3

from collections import defaultdict

def parse(crd):
  return [tuple([int(num) for num in s.strip().split(',')]) for s in crd.split('through')]

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    table = [[False]*1000 for i in range(1000)]
    for l in lines:
      if l.startswith('turn on'):
        crd = parse(l[8:])
        for x in range(crd[0][0], crd[1][0]+1):
          for y in range(crd[0][1], crd[1][1]+1):
            table[x][y] = True
      elif l.startswith('toggle'):
        crd = parse(l[7:])
        for x in range(crd[0][0], crd[1][0]+1):
          for y in range(crd[0][1], crd[1][1]+1):
            table[x][y] = not table[x][y]
      elif l.startswith('turn off'):
        crd = parse(l[9:])
        for x in range(crd[0][0], crd[1][0]+1):
          for y in range(crd[0][1], crd[1][1]+1):
            table[x][y] = False
    print(sum([sum([int(elem) for elem in l]) for l in table]))

if __name__=="__main__":
    main()
