from z3 import *

def parse_line(line):
    lights = line.split(']')[0][1:].strip()
    buttons = line.split(']')[1].split('{')[0].strip().split()
    buttons = [list(map(int, b.strip('()').split(','))) for b in buttons]
    joltage = list(map(int, line.split('{')[1].strip()[:-1].split(',')))
    return lights, buttons, joltage

with open('data/10/input') as f:
    total = 0
    for line in f:
        lights, buttons, joltage = parse_line(line)
        print(f"Lights: {lights}")
        print(f"Buttons: {buttons}")
        print(f"Joltage: {joltage}")

        X = [Int(f'x{i}') for i in range(len(buttons))]
        Constraints = []
        for x in X:
            Constraints.append(x >= 0)
        for i in range(len(lights)):
            sum_constraint = 0
            for j, button in enumerate(buttons):
                if i in button:
                    sum_constraint += X[j]
            Constraints.append(sum_constraint == joltage[i])
        
        s = Optimize()
        for c in Constraints:
            s.add(c)
        s.minimize(Sum(X))
        if s.check() == sat:
            m = s.model()
            result = [m.evaluate(X[i]).as_long() for i in range(len(buttons))]
            print(f"Solution: {result}")
            total += sum(result)
    
    print(f"Part 2: {total}")