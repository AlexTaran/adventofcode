#!/usr/bin/env python3

from collections import defaultdict

def proc(lines):
  res = [[None]*len(l) for l in lines]
  szx = len(lines[0])
  szy = len(lines)
  for x in range(szx):
    for y in range(szy):
      if lines[y][x] == '.':
        res[y][x] = '.'
        continue
      neibs = 0
      if x > 0     and lines[y][x-1] == '#': neibs += 1
      if x < szx-1 and lines[y][x+1] == '#': neibs += 1
      if y > 0     and lines[y-1][x] == '#': neibs += 1
      if y < szy-1 and lines[y+1][x] == '#': neibs += 1
      if x > 0 and y > 0  and lines[y-1][x-1] == '#': neibs += 1
      if x > 0 and y < szy-1 and lines[y+1][x-1] == '#': neibs += 1
      if x < szx-1 and y > 0  and lines[y-1][x+1] == '#': neibs += 1
      if x < szx-1 and y < szy-1 and lines[y+1][x+1] == '#': neibs += 1
      if neibs == 0:
        res[y][x] = '#'
      elif neibs >= 4:
        res[y][x] = 'L'
      else:
        res[y][x] = lines[y][x]

  return [''.join(l) for l in res]
  
def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    nxt = []
    while lines != nxt:
      nxt = proc(lines)
      lines, nxt = nxt, lines
    print(sum([len([ch for ch in l if ch == '#']) for l in lines]))

if __name__=="__main__":
    main()
