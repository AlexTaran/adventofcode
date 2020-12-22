#!/usr/bin/env python3


def main():
    lines = [l.strip() for l in open('input.txt')]
    pl1 = []
    pl2 = []
    cur = None
    for l in lines:
      if l == 'Player 1:':
        cur = pl1
        continue
      if l == 'Player 2:':
        cur = pl2
        continue
      if l == '': continue
      cur.append(int(l))
    while len(pl1) > 0 and len(pl2) > 0:
      if pl1[0] > pl2[0]:
        pl1 = pl1[1:] + [pl1[0], pl2[0]]
        pl2 = pl2[1:]
      else:
        pl2 = pl2[1:] + [pl2[0], pl1[0]]
        pl1 = pl1[1:]
    res = 0
    for i, elem in enumerate((pl1+pl2)[::-1]):
      res += (i+1)*elem
    print(res)

if __name__=="__main__":
    main()
