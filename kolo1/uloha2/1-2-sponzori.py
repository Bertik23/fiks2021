import os


def isInputYes(prompt):
    userInput = input(prompt)
    return userInput.lower() in [
        "yes", "y", "true", "t", "ye", "yeah", "ano", "a"
    ]


class Animal:
    """Class to store animals"""
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
                if matched.get(sponzor) is None or self.canMatch(
                    matched[sponzor], matched, seen
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


# if (
#     (x := os.path.exists("1-2-sponzori.vstup"))
#     or
#     isInputYes("Je vstup soubor? (y/N) ")
# ):
#     if not x:
#         with open("1-2-sponzori.vstup") as f:
#             inputString = f.read()
#     else:
#         with open(input("Cesta ke vstupu: ")) as f:
#             inputString = f.read()
# else:
inputString = ""
try:
    while (x := input()) not in ["", "\n"]:
        inputString += x + "\n"
except EOFError:
    pass

inputString = inputString.split("\n")
line = inputString[0].split(" ")
inputString = inputString[1:]
animalsNum = int(line[0])
sponzorsNum = int(line[1])

animals = {}
sponzors = {}

for _ in range(animalsNum):
    line = inputString[0].split(" ")
    inputString = inputString[1:]
    id_ = int(line[0])
    animals[id_] = (Animal(id_, line[1]))

for _ in range(sponzorsNum):
    line = inputString[0].split(" ")
    inputString = inputString[1:]
    sponzors[line[0]] = list(map(int, line[2:]))

for sponzor in sponzors:
    for i in sponzors[sponzor]:
        animals[i].potencialSponzors.append(sponzor)

g = Graph(animals, sponzors)

matching = g.findMatching()
if len(matching) == animalsNum:
    print("Ano")
else:
    print("Ne")
test = 10
for sponzor in sorted(matching, key=lambda x: matching[x].name):
    print(f"{{:<{max(map(len, [i.name for i in matching.values()]))}}}".format(
        matching[sponzor].name), sponzor
    )
