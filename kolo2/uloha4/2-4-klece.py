from random import randint


def generateInput(maxSize=10000, maxP=1000):
    out = [randint(1, maxSize) for _ in range(randint(1, maxSize))]
    return len(out), out, randint(1, maxP)


vstup = generateInput(100, 100)
# print(vstup)

animals = sorted(vstup[1])
p = vstup[2]

print(len(animals))

helpArr = []
animalsDict = {}
for i, a in enumerate(animals):
    try:
        while len(helpArr) and animals[helpArr[0]]+p < a:
            animalsDict[helpArr.pop(0)] = i-1
        helpArr.append(i)
    except Exception as e:
        print(helpArr[0])

for i, a in enumerate(animals):
    animalsDict[i] = animalsDict.get(i, len(animals)-1)

# print(list(zip(animalsDict.items(), animals)))

klece = []
queue = [0]
for a in queue:
    try:
        klece.append(animals[a:animalsDict[a]+1])
        if animalsDict[a]+1 < len(animals):
            queue.append(animalsDict[a]+1)
    except Exception as e:
        print(len(animals))
        print(len(animalsDict.keys()))
        raise

print(p, klece)
print(len(animals), sum(len(k) for k in klece))
