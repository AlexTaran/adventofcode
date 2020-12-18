#!/usr/bin/env python3

from collections import defaultdict

def mbint(i):
  if i.isdigit():
    return int(i)
  return i

def calc(s):
  s = [mbint(s) for s in list(s.replace(' ', ''))]
  # Fast programming!
  while len(s) != 1:
    done = False
    for i in range(2, len(s)):
      if type(s[i-2]) == int and s[i-1] == '+' and type(s[i]) == int:
        s = s[:i-2] + [s[i-2] + s[i]] + s[i+1:]
        done = True
        break
    if done: continue
    for i in range(2, len(s)):
      if type(s[i-2]) == int and s[i-1] == '*' and type(s[i]) == int:
        s = s[:i-2] + [s[i-2] * s[i]] + s[i+1:]
        done = True
        break
    if done: continue
    for i in range(2, len(s)):
      if s[i-2] == '(' and type(s[i-1]) == int and s[i] == ')':
        s = s[:i-2] + [s[i-1]] + s[i+1:]
        done = True
        break
    if done: continue
    break
  return s[0]

def main():
    lines = [l.strip() for l in open('input1.txt')]
    print(sum([calc(s) for s in lines]))

if __name__=="__main__":
    main()
