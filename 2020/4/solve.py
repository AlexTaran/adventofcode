#!/usr/bin/env python3

from collections import defaultdict

def handleData(lines):
  fieldData = {}
  for l in lines:
    for entry in l.split():
      k, v = entry.split(':')
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
