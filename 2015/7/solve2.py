#!/usr/bin/env python3

from collections import defaultdict

BASE = 65536

def main():
    lines = [l.strip() for l in open('input.txt')]
    exprs = {}
    for l in lines:
      expr, var = [s.strip() for s in l.split('->')]
      exprs[var] = expr
    exprs['b'] = '956'
    values = {}
    def getval(s):
      if s.isdigit(): return int(s)
      return calcvar(s)
    def getparams(expr, op):
      return [ getval(s.strip()) for s in expr.split(op)]

    def calcvar(var):
      if var in values: return values[var]
      expr = exprs[var]
      v = None
      if expr.isdigit():
        v = int(expr)
      elif expr in exprs.keys():
        v = getval(expr)
      elif 'RSHIFT' in expr:
        ps = getparams(expr, 'RSHIFT')
        for i in range(ps[1]):
          ps[0] //= 2
        v = ps[0]
      elif 'LSHIFT' in expr:
        ps = getparams(expr, 'LSHIFT')
        for i in range(ps[1]):
          ps[0] *= 2
          ps[0] %= BASE
        v = ps[0]
      elif 'AND' in expr:
        ps = getparams(expr, 'AND')
        v = ps[0] & ps[1]
      elif 'OR' in expr:
        ps = getparams(expr, 'OR')
        v = ps[0] | ps[1]
      elif 'NOT' in expr:
        v = BASE - 1 - getval(expr.split('NOT')[1].strip())
      else:
        print(expr, var)
      values[var] = v
      return v
    print(calcvar('a'))

if __name__=="__main__":
    main()
