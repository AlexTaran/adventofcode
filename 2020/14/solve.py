#!/usr/bin/env python3

from collections import defaultdict

def main():
    lines = [l.strip() for l in open('input.txt')]
    mask = ''
    mem = {}
    for l in lines:
      cmd, val = l.split(' = ')
      if cmd == 'mask':
        mask = val
      else:
        addr = int(cmd[4:-1])
        bn = list(bin(int(val))[2:])
        while len(bn) < len(mask):
          bn = ['0'] + bn
        for i, m in enumerate(mask):
          if m != 'X':
            bn[i] = m
        mem[addr] = int(''.join(bn), 2)
    print(sum(mem.values()))

if __name__=="__main__":
    main()
