#!/usr/bin/env python3

from collections import defaultdict

def isnice(s):
  if not any([s[i]==s[i+2] for i in range(len(s)-2)]):
    return False
  for i in range(len(s)-1):
    pr = s[i:i+2]
    for j in range(i+2, len(s)-1):
      if s[j:j+2] == pr:
        return True
  return False

def main():
    lines = [l.strip() for l in open('input.txt')]
    print(len([l for l in lines if isnice(l)]))

if __name__=="__main__":
    main()
