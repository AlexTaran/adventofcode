#!/usr/bin/env python3

def sim(grid):
  cands = set()
  dlt = [-1, 0, 1]
  for x, y, z in grid:
    for dx in dlt:
      for dy in dlt:
        for dz in dlt:
          cands.add( (x + dx, y + dy, z + dz) )
  newgrid = set()
  for x, y, z in cands:
    nc = 0
    for dx in dlt:
      for dy in dlt:
        for dz in dlt:
          if dx != 0 or dy != 0 or dz != 0:
            if (x + dx, y + dy, z + dz) in grid:
              nc += 1
    if nc == 3 and (x, y, z) not in grid:
      newgrid.add( (x, y, z) )
    if nc in [2, 3] and (x, y, z) in grid:
      newgrid.add( (x, y, z) )

  return newgrid

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    grid = set()
    for y, l in enumerate(lines):
      for x, ch in enumerate(l):
        if ch == '#':
          grid.add( (x, y, 0) )
    for i in range(6):
      grid = sim(grid)
    print(len(grid))

if __name__=="__main__":
    main()
