#!/usr/bin/env python3

from collections import defaultdict

def calcop(left, op, right):
  if op == '+':
    return left + right
  else:
    return left * right

def calc(s):
  s = s.replace(' ', '')
  stk = []
  for elem in s:
    if elem in '+*(':
      stk.append(elem)
    elif elem.isdigit():
      z = int(elem)
      if len(stk) == 0 or stk[-1] == '(':
        stk.append(z)
      else:
        op = stk.pop()
        prev = stk.pop()
        stk.append(calcop(prev, op, z))
    elif elem == ')':
      insd = stk.pop()
      stk.pop()  # '('
      if len(stk) == 0 or stk[-1] == '(':
        stk.append(insd)
      else:
        op = stk.pop()
        prev = stk.pop()
        stk.append(calcop(prev, op, insd))
  return stk[0]

def main():
    lines = [l.strip() for l in open('input.txt')]
    print(sum([calc(s) for s in lines]))

if __name__=="__main__":
    main()
