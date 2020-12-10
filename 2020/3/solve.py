#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    lines = [l.strip() for l in open('input.txt')]
    curx = 0
    cury = 0
    while cury != len(lines)-1:
        curx = (curx + 3) % len(lines[0])
        cury = (cury + 1) % len(lines)
        if lines[cury][curx] == '#':
            res += 1
    print(res)

if __name__=="__main__":
    main()
