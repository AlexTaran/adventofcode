#!/usr/bin/env python3

from collections import defaultdict

data="""
children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1
"""

def main():
    res = 0
    eth = dict([tuple(l.split(': ')) for l in data.split('\n') if l.strip() != ''])
    lines = [l.strip() for l in open('input.txt')]
    for l in lines:
      sue, params = l.split(': ', 1)
      sue = int(sue.split()[1])
      params = dict([tuple(s.split(': ')) for s in params.split(', ')])
      fits = True
      for k, v in params.items():
        if k not in eth: continue
        if k in ['cats', 'trees']:
          if int(eth[k]) >= int(v):
            fits = False
            break
        elif k in ['pomeranians', 'goldfish']:
          if int(eth[k]) <= int(v):
            fits = False
            break
        else:
          if eth[k] != v:
            fits = False
            break
      if fits:
        print(sue)
      pass

if __name__=="__main__":
    main()
