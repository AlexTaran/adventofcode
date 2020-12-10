#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    for l in lines:
        rng, lt, pw = l.split(' ')
        mn, mx = [int(x) for x in rng.split('-')]
        lt = lt[0]
        #print(mn, mx, lt, pw)
        cnt = sum([1 for idx in [mn, mx] if pw[idx-1] == lt])
        if cnt == 1:
            res += 1
    print(res)

if __name__=="__main__":
    main()
