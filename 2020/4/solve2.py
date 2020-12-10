#!/usr/bin/env python3

from collections import defaultdict

def handleData(lines):
  fieldData = {}
  for l in lines:
    for entry in l.split():
      k, v = entry.split(':')
      if k == 'byr':
        if int(v) > 2002 or int(v) < 1920: return False
      elif k == 'iyr':
        if int(v) > 2020 or int(v) < 2010: return False
      elif k == 'eyr':
        if int(v) > 2030 or int(v) < 2020: return False
      elif k == 'hgt':
        if v.endswith('in'):
          val = int(v[:-2])
          if val < 59 or val > 76: return False
        elif v.endswith('cm'):
          val = int(v[:-2])
          if val < 150 or val > 193: return False
        else: return False
      elif k == 'hcl':
        if v[0] != '#': return False
        if len(v[1:]) != 6: return False
        if any([ch not in '0123456789abcdef' for ch in v[1:]]): return False
      elif k == 'ecl':
        if v not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']: return False
      elif k == 'pid':
        if len(v) != 9: return False
        if any([ch not in '0123456789' for ch in v]): return False
      fieldData[k] = v
  return len(fieldData) == 8 or (len(fieldData) == 7 and 'cid' not in fieldData)

def main():
    lines = [l.strip() for l in open('input.txt')] + ['']
    data = []
    res = 0
    for l in lines:
      if len(l) == 0:
        if handleData(data): res += 1
        data = []
        continue
      data.append(l)
    print(res)

if __name__=="__main__":
    main()
