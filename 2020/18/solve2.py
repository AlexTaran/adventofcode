#!/usr/bin/env python3

from collections import defaultdict

def mbint(i):
  if i.isdigit():
    return int(i)
  return i

def calc(s):
  s = [mbint(s) for s in list(s.replace(' ', ''))]

  def calcProd(start):
    res = 1
    while start < len(s) and s[start] != ')':
      sm, nstart = calcSum(start)
      res *= sm
      if nstart < len(s) and s[nstart] == '*':
        start = nstart + 1
      else:
        start = nstart
        break
    return (res, start)
    
  def calcSum(start):
    res = 0
    while start < len(s) and s[start] != ')':
      sm, nstart = calcElem(start)
      res += sm
      if nstart < len(s) and s[nstart] == '+':
        start = nstart + 1
      else:
        start = nstart
        break
    return (res, start)

  def calcElem(start):
    if type(s[start]) == int:
      return (s[start], start + 1)
    prod, fin = calcProd(start + 1) # skip '('
    return (prod, fin + 1) # skip ')'

  return calcProd(0)[0]

def main():
    lines = [l.strip() for l in open('input.txt')]
    print(sum([calc(s) for s in lines]))

if __name__=="__main__":
    main()
