from copy import deepcopy


def dijkstra(graph, start, end):
    # print(graph, start, end)
    queue = [start]
    distancesGraph = {i: float("inf") for i in graph}
    distancesGraph[start] = 0
    prevNodes = {}
    for q in queue:
        # print(q)

        for n in graph[q]:
            # print(n, graph[q], q)
            if distancesGraph[q] + n[0] < distancesGraph[n[1]]:
                queue.append(n[1])
                prevNodes[n[1]] = q
                distancesGraph[n[1]] = n[0] + distancesGraph[q]

    if distancesGraph[end] == float("inf"):
        return [], 0
    path = findPath(prevNodes, start, end)
    price = 0
    for i, node in enumerate(path):
        if node == end:
            break
        # print(node, graph[node], path[i+1])
        # for x in graph[node]:
            # print(float("inf") if x[1] != path[i+1] else x[0])
        prices = list(
            map(
                lambda x: float("inf")
                if x[1] != path[i+1] else x[0], graph[node]
            )
        )
        price += min(prices)
    return path, price


def findPath(graph, start, end):
    # print(graph)
    n = end
    path = [end]
    while n in graph:
        n = graph[n]
        path.insert(0, n)
    return path
    # print(min(graph[end], key=lambda x: distancesGraph[x[1]]))


with open("2-3-delnici.vstup", encoding="utf-8") as f:
    inputString = f.read()

inputString = inputString.split("\n")
numOfProblems = int(inputString[0])
inputString = inputString[1:]

with open("2-3-delnici-ven.txt", "w", newline="\n", encoding="utf-8") as f:
    for _ in range(numOfProblems):
        numOfLangs = int(inputString[0])
        inputString = inputString[1:]
        langs = {inputString[lI]: [] for lI in range(numOfLangs)}
        inputString = inputString[numOfLangs:]

        numOfTranslators = int(inputString[0])
        inputString = inputString[1:]
        for tI in range(numOfTranslators):
            translator = inputString[tI].split(" ")[1:]
            for lang in translator[1:]:
                toLangs = deepcopy(translator[1:])
                toLangs.remove(lang)
                langs[lang].extend(
                    [(int(translator[0]), toLang) for toLang in toLangs]
                )

        inputString = inputString[numOfTranslators:]
        path, price = dijkstra(langs, *(inputString[0].split(" ")))
        inputString = inputString[1:]
        if path == []:
            print("Takove prekladatele nemame.")
            f.write("Takove prekladatele nemame.\n")
        else:
            print(f"To nas bude stat {price},-.")
            f.write(f"To nas bude stat {price},-.\n")
            print(f"Pocet prekladu: {len(path)-1}.")
            f.write(f"Pocet prekladu: {len(path)-1}.\n")
            for lang in path:
                print(lang)
                f.write(lang+"\n")
