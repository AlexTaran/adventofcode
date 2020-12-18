#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    # Luckily string values do not have numbers!
    for l in lines:
      nb = ''
      for ch in l:
        if ch.isdigit() or ch == '-':
          nb = nb + ch
        else:
          if nb != '':
            res += int(nb)
          nb = ''
    print(res)

if __name__=="__main__":
    main()
