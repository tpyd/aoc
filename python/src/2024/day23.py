import copy
import functools

data = """kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
"""

with open("input/2024/day23.txt") as f:
    data = f.read()

raw = copy.deepcopy(data)
data = data.strip().split("\n")
data = [sorted(d.split("-")) for d in data]
data = sorted(data, key=lambda x: x[0])

connections = {}

for a, b in data:
    if a not in connections:
        connections[a] = []

    if b not in connections:
        connections[b] = []

    connections[a].append(b)
    connections[b].append(a)

#raw_connections = copy.deepcopy(connections)
connections = {k: v for k, v in connections.items() if len(v) >= 3}
#[print(k, v) for k, v in connections.items()]

triplets = set()

for a in connections:
    duo = connections[a]

    for i, b in enumerate(duo[:-1]):
        for c in duo[i+1:]:
            if b in connections[c] and c in connections[b]:
                triplet = tuple(sorted([a, b, c]))
                triplets.add(triplet) 

#print(triplets)
#print(len(triplets))

filtered = []
for triplet in triplets:
    for t in triplet:
        if t.startswith("t"):
            filtered.append(triplet)
            break

print(len(filtered))


# Part 2
data = raw.strip().split("\n")
connections = [d.split("-") for d in data]
unique = list(set([node for connection in data for node in connection.split("-")]))

@functools.cache
def extend(nodes):
    # Check if we are at max depth
    if len(nodes) == len(unique):
        return nodes

    all_connections = []

    # Get all new connections for the nodes
    for node in nodes:
        connected_nodes = list(set([new for c in data for new in c.split("-") if node in c]))
        all_connections.extend(connected_nodes)

    all_connections = list(set(all_connections))

    # Try to add one node
    biggest = list(nodes)

    for new_node in all_connections:
        # Check that the new node is conneted to all existing nodes
        is_connected = True
        for node in nodes:
            if f"{node}-{new_node}" not in data and f"{new_node}-{node}" not in data:
                is_connected = False

        if not is_connected:
            continue

        # Try to go deeper
        new_biggest = extend(tuple(sorted(nodes + (new_node,))))

        if len(new_biggest) > len(biggest):
            biggest = new_biggest

    # Could not find a bigger network
    return biggest


biggest = []

for i, node in enumerate(unique):
    print(f"{i} / {len(unique)}")
    candidate = extend((node,))

    if len(candidate) > len(biggest):
        biggest = candidate

print(",".join(sorted(biggest)))

