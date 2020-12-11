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
      deltas = [(0, 1), (1, 0), (-1, 0), (0, -1), (1, 1), (-1, -1), (1, -1), (-1, 1)]
      for dy, dx in deltas:
        cx = x + dx
        cy = y + dy
        while cx >= 0 and cy >= 0 and cx < szx and cy < szy and lines[cy][cx] == '.':
          cx = cx + dx
          cy = cy + dy
        if cx >= 0 and cy >= 0 and cx < szx and cy < szy and lines[cy][cx] == '#':
          neibs += 1
      if neibs == 0:
        res[y][x] = '#'
      elif neibs >= 5:
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
