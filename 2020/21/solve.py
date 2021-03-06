#!/usr/bin/env python3

from collections import defaultdict

def main():
    lines = [l.strip() for l in open('input.txt')]
    allingreds = set()
    allallergs = set()

    candidates = defaultdict(set)

    lns = []
    for l in lines:
      ingreds, allergs = l[:-1].split(' (contains ')
      ingreds = ingreds.split()
      allergs = allergs.split(', ')
      allingreds.update(ingreds)
      allallergs.update(allergs)
      lns.append([ingreds, allergs])
    for allerg in allallergs:
      candidates[allerg] = allingreds.copy()
    for ingreds, allergs in lns:
      for a in allergs:
        candidates[a].intersection_update(ingreds)
    visited = set()
    while True:
      rem = None
      for c, ings in candidates.items():
        if len(ings) == 1 and c not in visited:
          visited.add(c)
          rem = c
          break
      if rem == None: break
      for c, ings in candidates.items():
        if c != rem: candidates[c].discard(list(candidates[rem])[0])
    bads = set()
    for c, ings in candidates.items(): bads.update(ings)
    res = 0
    for ingreds, allergs in lns:
      for ing in ingreds:
        if ing not in bads: res += 1
    print(res)

if __name__=="__main__":
    main()
