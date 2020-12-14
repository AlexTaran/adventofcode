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
        bn = list(bin(addr)[2:])
        while len(bn) < len(mask): bn = ['0'] + bn
        mx = len([ch for ch in mask if ch == 'X'])
        for m in range(2**mx):
          bzn = bn[:]
          bits = bin(m)[2:]
          while len(bits) < mx: bits = '0' + bits
          j = 0
          for i, m in enumerate(mask):
            if m == 'X':
              bzn[i] = bits[j]
              j += 1
            elif m == '1':
              bzn[i] = '1'
          mem[int(''.join(bzn), 2)] = int(val)
    print(sum(mem.values()))

if __name__=="__main__":
    main()
