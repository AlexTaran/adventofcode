#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    visited = [False] * len(lines)
    curr = 0
    acc = 0
    while not visited[curr]:
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
    print(acc)

if __name__=="__main__":
    main()
