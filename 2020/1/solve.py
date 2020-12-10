#!/usr/bin/env python3

def main():
    nums = [int(l.strip()) for l in open('input.txt')]
    print(nums)
    for i, x in enumerate(nums):
        for j, y in enumerate(nums):
            if x+y == 2020:
                print("Found ", i, j, " prod = ", x*y)

if __name__=="__main__":
    main()
