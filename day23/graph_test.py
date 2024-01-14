class Node:
    def __init__(self, name):
        self.connections = {}
        self.name = name

    def add_connection_to(self, node, weight):
        self.connections[node] = weight

    def add_connection_bidirectional(self, node, weight_to, weight_from = None):
        if weight_from is None:
            weight_from = weight_to
        self.add_connection_to(node, weight_to)
        node.add_connection_to(self, weight_from)

a = Node("a")
b = Node("b")
c = Node("c")
a.add_connection_bidirectional(b, 3)
b.add_connection_bidirectional(c, 4)
for n, w in a.connections.items():
    print(n.name)
    for x, y in n.connections.items():
        print(x.name)
        print(y)