class Animal:
    def __init__(self, id_, name):
        self.id = id_
        self.name = name
        self.potencialSponzors = []

    def __str__(self):
        return (
            f"{self.name} - {self.id} - "
            f"{self.potencialSponzors}"
        )

    def __repr__(self):
        return str(self)


class Graph:
    def __init__(self, animalDict: dict, sponzorDict: dict):
        self.animalDict = animalDict
        self.animals: list[Animal] = list(animalDict.values())
        self.sponzorDict = sponzorDict
        self.sponzors = list(sponzorDict.keys())
        self.nodes = self.animals + self.sponzors
        self.graph = {}
        for a in self.animals:
            self.graph[a.id] = a.potencialSponzors
        for s in self.sponzors:
            self.graph[s] = self.sponzorDict[s]

    def canMatch(
        self,
        animal: Animal,
        matched: dict[str, Animal],
        seen: dict[str, bool]
    ):
        for sponzor in self.sponzors:
            if (
                animal.id in self.graph[sponzor]
                and not seen.get(sponzor, False)
            ):
                seen[sponzor] = True
                if (
                    matched.get(sponzor, None) is None
                    or self.canMatch(matched[sponzor], matched, seen)
                ):
                    matched[sponzor] = animal
                    return True
        return False

    def findMatching(self):
        match = {}
        for animal in self.animals:
            seen = {}
            self.canMatch(animal, match, seen)
        return match


with open("1-2-sponzori.vstup") as f:
    inputString = f.read()

inputString = inputString.split("\n")
line = inputString[0].split(" ")
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
    sponzors[line[0]] = list(map(int, line[2:]))

for sponzor in sponzors:
    for i in sponzors[sponzor]:
        animals[i].potencialSponzors.append(sponzor)

print(animals)
print(sponzors)
g = Graph(animals, sponzors)

matching = g.findMatching()
if len(matching) == animalsNum:
    print("Ano")
else:
    print("Ne")
for sponzor in sorted(matching, key=lambda x: matching[x].name):
    print(matching[sponzor].name, sponzor)
