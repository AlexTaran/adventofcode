#!/usr/bin/env python3

def combat(pl1, pl2):
  def w1():
    nonlocal pl1
    nonlocal pl2
    pl1 = pl1[1:] + [pl1[0], pl2[0]]
    pl2 = pl2[1:]
  def w2():
    nonlocal pl1
    nonlocal pl2
    pl2 = pl2[1:] + [pl2[0], pl1[0]]
    pl1 = pl1[1:]
  history = set([(tuple(pl1), tuple(pl2))])
  while len(pl1) > 0 and len(pl2) > 0:
    if len(pl1) > pl1[0] and len(pl2) > pl2[0]:
      res1, res2 = combat(pl1[1:pl1[0]+1], pl2[1:pl2[0]+1])
      if len(res1) == 0:
        w2()
      else:
        w1()
    elif pl1[0] > pl2[0]:
      w1()
    else:
      w2()
    pos = (tuple(pl1), tuple(pl2))
    if pos in history: return [pl1, []]
    history.add(pos)
  return [pl1, pl2]

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
    pl1, pl2 = combat(pl1, pl2)
    res = 0
    for i, elem in enumerate((pl1+pl2)[::-1]):
      res += (i+1)*elem
    print(res)

if __name__=="__main__":
    main()
