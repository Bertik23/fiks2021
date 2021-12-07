VISUALISE = True


def findPath(graph, start, end):
    visited = set()
    queue = [start]
    for q in queue:
        for n in graph[q]:
            if n == end:
                return True
            if n not in visited:
                queue.append(n)
                visited.add(n)
    return False


with open("2-5-prohlidkove-okruhy.vstup", encoding="utf-8") as f:
    inputString = f.read()

inputString = inputString.split("\n")

with open(
    "2-5-prohlidkove-okruhy.out",
    "w",
    newline="\n",
    encoding="utf-8"
) as f:
    numOfNodes, numOfEdges, numOfQuestions = map(
        int,
        inputString[0].split(" ")
    )
    inputString = inputString[1:]
    graph = {lI: [] for lI in range(1, numOfNodes+1)}

    print(graph)
    for eI in range(numOfEdges):
        a, b = map(int, inputString[eI].split(" "))
        graph[a].append(b)

    inputString = inputString[numOfEdges:]

    questions = []
    for qI in range(numOfQuestions):
        a, b = map(int, inputString[qI].split(" "))
        questions.append((a, b))

    print(graph)

    if VISUALISE:
        from pyvis.network import Network
        net = Network()

        edges = []
        for start, targets in graph.items():
            for end in targets:
                edges.append((start, end))

        print(edges)

        net.add_nodes(graph.keys())
        for edge in edges:
            net.add_edge(*edge)

        net.show("2-5-prohlidkove-okruhy.graph.html")

    for (start, end) in questions:
        if findPath(graph, start, end):
            print("Cesta existuje")
        else:
            print("Cesta neexistuje")
