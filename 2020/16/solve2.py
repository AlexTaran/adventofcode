#!/usr/bin/env python3

from collections import defaultdict
from itertools import product

def main():
    res = 1
    lines = [l.strip() for l in open('input.txt')]
    state = 'fields'
    fields = {}
    mt = None
    nbt = []
    for l in lines:
      if l == '': continue
      if l == 'nearby tickets:':
        state = 'nbt'
        continue
      if l == 'your ticket:':
        state = 'mt'
        continue
      if state == 'fields':
        name, vals = l.split(': ')
        diaps = vals.split(' or ')
        numbers = set()
        for diap in diaps:
          mn, mx = list(map(int, diap.split('-')))
          for x in range(mn, mx+1):
            numbers.add(x)
        fields[name] = numbers
      if state == 'mt':
        mt = list(map(int, l.split(',')))
      if state == 'nbt':
        nbt.append(list(map(int, l.split(','))))
    fullset = set()
    for name, vals in fields.items():
      for v in vals: fullset.add(v)
    nbt = [t for t in nbt if all([v  in fullset for v in t])]
    fieldvalues = {}
    for i in range(len(mt)):
      fieldvalues[i] = set([mt[i]])
      for t in nbt: fieldvalues[i].add(t[i])
    candidatefields = [None]* len(mt)
    for i in range(len(mt)):
      candidatefields[i] = set()
      for name, vals in fields.items():
        if all([v in vals for v in fieldvalues[i]]):
          candidatefields[i].add(name)
    for f in candidatefields:
      print(f)


if __name__=="__main__":
    main()
