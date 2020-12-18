#!/usr/bin/env python3

from collections import defaultdict
import json

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    ob = json.loads(lines[0])
    def visit(o):
      nonlocal res
      if type(o) == int:
        res += o
      elif type(o) == list:
        for elem in o:
          visit(elem)
      elif type(o) == dict:
        if 'red' not in o.values():
          for k, v in o.items():
            visit(v)
    visit(ob)
    print(res)

if __name__=="__main__":
    main()
