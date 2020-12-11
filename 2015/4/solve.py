#!/usr/bin/env python3

from hashlib import md5

def main():
  inp = 'yzbqklnj'
  i = 1
  while True:
    s = md5((inp+str(i)).encode()).hexdigest()
    if s.startswith('00000'):
      print(i)
      break
    i += 1

if __name__=="__main__":
    main()
