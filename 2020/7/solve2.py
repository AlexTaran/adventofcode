#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    graph = defaultdict(list)
    invgraph = defaultdict(list)
    for l in lines:
      bag, cont = l[:-1].split(' bags contain ')
      if cont == 'no other bags':
        cont = []
      else:
        cont = [tuple(elem.rsplit(' ', 1)[0].split(' ', 1)) for elem in cont.split(', ')]
        cont = [(int(cnt), v) for cnt, v in cont]
      graph[bag].extend(cont)
      for cnt, v in cont:
        invgraph[v].append(bag)
    counters = {}
    def count(name):
      if name in counters: return counters[name]
      cnt = 1
      for c, nm in graph[name]:
        cnt += count(nm) * c
      counters[name] = cnt
      return cnt
    print(count('shiny gold') - 1)

if __name__=="__main__":
    main()
