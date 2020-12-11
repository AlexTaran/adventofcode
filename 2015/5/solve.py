#!/usr/bin/env python3

from collections import defaultdict

def isnice(s):
  vow = 'aeiou'
  if sum([1 for ch in s if ch in vow]) < 3:
    return False
  if not any([s[i]==s[i+1] for i in range(len(s)-1)]):
    return False
  if any([c in s for c in ['ab', 'cd', 'pq', 'xy']]):
    return False
  return True

def main():
    lines = [l.strip() for l in open('input.txt')]
    print(len([l for l in lines if isnice(l)]))

if __name__=="__main__":
    main()
