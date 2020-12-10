#!/usr/bin/env python3

def main():
    nums = [int(l.strip()) for l in open('input.txt')]
    print(nums)
    print(len(nums))
    for i, x in enumerate(nums):
        for j, y in enumerate(nums):
            for k, z in enumerate(nums):
                if x+y+z == 2020:
                    print("Found ", i, j, k, " prod = ", x*y*z)

if __name__=="__main__":
    main()
