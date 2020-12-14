#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    tt = int(lines[0])
    tms = [int(s) for s in lines[1].split(',') if s != 'x']
    wts = [(t - tt % t) % t for t in tms]
    i = wts.index(min(wts))
    print(tms[i]*wts[i])

if __name__=="__main__":
    main()
