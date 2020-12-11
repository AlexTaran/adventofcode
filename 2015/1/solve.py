#!/usr/bin/env python3

from collections import defaultdict

def main():
    res = 0
    line = [l.strip() for l in open('input.txt')][0]
    print(sum([{'(': 1, ')': -1}[ch] for ch in line]))

if __name__=="__main__":
    main()
