#!/usr/bin/env python3

from collections import defaultdict

def run(lines):
    visited = [False] * len(lines)
    curr = 0
    acc = 0
    while curr < len(lines) and not visited[curr]:
      visited[curr] = True
      cmd, arg = lines[curr].split(' ')
      arg = int(arg)
      if cmd == 'nop':
        curr += 1
      elif cmd == 'jmp':
        curr = curr + arg
      elif cmd == 'acc':
        curr += 1
        acc += arg
    if curr == len(lines):
      return True, acc
    else:
      return False, acc

def main():
    lines = [l.strip() for l in open('input.txt')]
    for i in range(len(lines)):
      if lines[i].startswith('acc'): continue
      z = lines[:]
      if z[i].startswith('nop'):
        z[i] = z[i].replace('nop', 'jmp')
      else: 
        z[i] = z[i].replace('jmp', 'nop')
      res, acc = run(z)
      if res:
        print(acc)

if __name__=="__main__":
    main()
