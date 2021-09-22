from copy import deepcopy
# from multiprocessing import Pool

class NotDAP(BaseException):
    pass


class Graph:
    def __init__(self, inDict):
        self.inDict = inDict
        self.nodes = self.getNodes(inDict)
        self.edges = self.getEdges(inDict)
        self.tempMarks = []
        self.permMarks = []


    @classmethod
    def getNodes(self, inDict: dict):
        nodes = []
        for i in inDict.values():
            for j in i:
                nodes.append(j)
        return list(set(list(inDict.keys()) + nodes))

    @classmethod
    def getEdges(self, inDict: dict):
        edges = []
        for a in inDict:
            for n in inDict[a]:
                edges.append((n, a))
        return edges

    @property
    def startNodes(self):
        out = []
        for n in self.nodes:
            if len(self[n]) == 0:
                out.append(n)
        return out

    def topoSortedStr(self):
        s = self.topoSort2()
        out = ""
        for i in s:
            out += f"{i} "
        return out.strip()

    def topoSort(self):
        s = deepcopy(self.startNodes)
        copyGraph = Graph(self.inDict)
        sortedNodes = []
        while len(s):
            n = s.pop()
            sortedNodes.append(n)
            for e in copyGraph[n, True]:
                m = e[1]
                copyGraph.edges.remove(e)
                if len(copyGraph[n]) == 0:
                    s.append(m)
        return sortedNodes

    def topoSort2(self):
        sortedNodes = []
        def visit(n):
            if n in self.permMarks:
                return
            if n in self.tempMarks:
                raise NotDAP
            # print(f"Visiting {n}, {self[n, True]}")
            self.tempMarks.append(n)
            for e in self[n, True]:
                visit(e[1])

            self.tempMarks.remove(n)
            self.permMarks.append(n)
            sortedNodes.insert(0, n)
        while len(self.permMarks) < len(self.nodes):
            node = list(set(self.nodes) - set(self.permMarks))[0]
            visit(node)
        self.tempMarks = []
        self.permMarks = []
        return sortedNodes

    
    def getGraphFromStartNode(self, startNode):
        d = {}
        nodes = [startNode]
        for n in nodes:
            try:
                d[n] = [i[0] for i in self[n]]
                # print(self[n], n)
                # nodes.extend([i[0] for i in self[n] if i[0] not in nodes])
                for n2 in [i[0] for i in self[n] if i[0]]:
                    if n2 not in nodes:
                        nodes.append(n2)
            except IndexError:
                continue
        return d


    def __getitem__(self, key):
        if type(key) == tuple:
            key, outBound = key
        else:
            outBound = False
        out = []
        for n in self.edges:
            if outBound:
                if n[0] == key:
                    out.append(n)
            else:
                if n[1] == key:
                    out.append(n)
        return out


if __name__ == "__main__":
    import time
    startTime = time.time()
    dictCreationTime = 0
    with open("1-1-byrokracie.vstup") as f:
        inputString = f.read()
    inputString = inputString.split("\n")
    # print(inputString)
    numOfProblems = int(inputString[0])
    inputString = inputString[1:]
    print(numOfProblems)
    with open("1-1-byrokracie.out", "w", encoding="utf-8") as f:
        # with Pool(numOfProblems//2) as pool:
        for i in range(numOfProblems):
            line = inputString[0].split(" ")
            inputString = inputString[1:]
            auths = int(line[0])
            lines = int(line[1])
            authID = line[2]
            authDict = {}
            dictStartTime = time.time()
            for j in range(lines):
                line = inputString[0].split(" ")
                inputString = inputString[1:]
                id_ = line[0]
                authDict[id_] = authDict.get(id_, []) + [line[1]]
            dictCreationTime += time.time() - dictStartTime
            g = Graph(authDict)
            newG = Graph(g.getGraphFromStartNode(authID))
            try:
                printStr = f"pujde to {newG.topoSortedStr()}"
                f.write(f"{printStr}\n")
                print(printStr)
            except NotDAP:
                f.write("ajajaj\n")
                print("ajajaj")
    print(f"Took {time.time() - startTime}\nDict creation {dictCreationTime}")
