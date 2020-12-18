#!/usr/bin/env python3

from collections import defaultdict
from itertools import permutations

def main():
    lines = [l.strip() for l in open('input.txt')]
    graph = defaultdict(dict)
    for l in lines:
      cities, dist = l.split(' = ')
      f, t = cities.split(' to ')
      graph[f][t] = int(dist)
      graph[t][f] = int(dist)
    res = 10**10
    for p in permutations(graph.keys()):
      tot = 0
      for i in range(1, len(p)):
        tot += graph[p[i-1]][p[i]]
      res = min(res, tot)
    print(res)

if __name__=="__main__":
    main()
