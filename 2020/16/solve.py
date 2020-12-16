#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
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
    for t in nbt:
      for v in t:
        if v not in fullset:
          res += v
    print(res)


if __name__=="__main__":
    main()
