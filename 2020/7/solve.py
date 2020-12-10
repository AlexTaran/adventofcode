#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    graph = defaultdict(list)
    invgraph = defaultdict(list)
    for l in lines:
      bag, cont = l[:-1].split(' bags contain ')
      cont = [tuple(elem.rsplit(' ', 1)[0].split(' ', 1)) for elem in cont.split(', ')]
      graph[bag].extend([v for cnt, v in cont])
      for cnt, v in cont:
        invgraph[v].append(bag)
    visited = set(['shiny gold'])
    q = ['shiny gold']
    while len(q) > 0:
      v = q[0]
      q = q[1:]
      for bs in invgraph[v]:
        if bs not in visited:
          visited.add(bs)
          q.append(bs)
    print(len(visited)-1)

if __name__=="__main__":
    main()
