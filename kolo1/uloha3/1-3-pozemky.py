from time import time

startTime = time()

with open("1-3-pozemky.vstup") as f:
    inputString = f.read()

inputString = inputString.split("\n")
numOfProblems = int(inputString[0])
inputString = inputString[1:]
print(numOfProblems)
with open("1-3-pozemky.out", "w", encoding="utf-8") as f:
    for i in range(numOfProblems):
        line = inputString[0].split(" ")
        inputString = inputString[1:]
        lines = int(line[0])
        sumOfAreas = 0
        for lineI in range(lines):
            line = inputString[0].split(" ")
            inputString = inputString[1:]
            sumOfAreas += int(line[1]) * int(line[2])
        print(sumOfAreas)
        f.write(f"{sumOfAreas}\n")

print(f"Took {time() - startTime}")
