class Animal:
    def __init__(self, id_, name):
        self.id = id_
        self.name = name
        self.potencialSponzors = []
        self.sponzor = None
    def __str__(self):
        return f"{self.name} - {self.id} - {self.potencialSponzors} - {self.sponzor}"
    def __repr__(self):
        return str(self)

with open("1-2-sponzori.vstup") as f:
    inputString = f.read()

inputString = inputString.split("\n")
print(inputString)
line = inputString[0].split(" ")
print(line)
inputString = inputString[1:]
animalsNum = int(line[0])
sponzorsNum = int(line[1])

animals = {}
sponzors = {}

for i in range(animalsNum):
    line = inputString[0].split(" ")
    inputString = inputString[1:]
    id_ = int(line[0])
    animals[id_] = (Animal(id_, line[1]))

for i in range(sponzorsNum):
    line = inputString[0].split(" ")
    inputString = inputString[1:]
    sponzors[line[0]] = list(map(int, line[1:]))

for sponzor in sponzors:
    for i in sponzors[sponzor]:
        animals[i].potencialSponzors.append(sponzor)
print(animals)
sponzoredAnimals = []
while len(sponzors):
    a = animals[min(animals, key=lambda x: len(animals[x].potencialSponzors))]
    print(a)
    a.sponzor = min(a.potencialSponzors, key=lambda x: len([animal for animal in animals.values() if x in animal.potencialSponzors]))
    sponzors.pop(a.sponzor)
    for animal in animals:
        try:
            animals[animal].potencialSponzors.remove(a.sponzor)
        except ValueError:
            pass
    sponzoredAnimals.append(a)
    animals.pop(a.id)

print(sponzoredAnimals)
