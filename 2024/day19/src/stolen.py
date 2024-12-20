# Source:
# https://www.reddit.com/r/adventofcode/comments/1hhlb8g/comment/m2ygqdv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button

from collections import defaultdict                                                         
import sys                                                                                  
                                              
# towels = set(input().split(', '))     
# input()                                       

with open("./puzzle/test.txt") as f:
    contents = f.readlines()

towels = set([x.strip() for x in contents[0].split(", ")])
print(towels)

max_towel_size = max([len(t) for t in towels])                                              
                                                                                            
result = 0                                                                                  
for pattern in contents[2:]:                                                                   
    print(pattern)
    counts = defaultdict(lambda: 0)                                                         
    counts[0] = 1                                                                           
    for i in range(1, len(pattern)):
        print("i:", i)                                                        
        for j in range(max(0, i - max_towel_size), i):                                      
            print("j:", j)                                                        
            if pattern[j:i] in towels:
                print(pattern[j:i])                                                      
                counts[i] += counts[j]                                                      
    print(counts)
    result += counts[len(pattern) - 1]                                                      
print(result)
