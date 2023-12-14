from copy import deepcopy

D = [[a for a in l] for l in open(0).read().split("\n") if l != ""]
d = 0
def shift_north(G):
  for i in range(len(G[0])):
    last_free = 0
    for j in range(len(G)):
      if G[j][i] == '#':
        last_free = j+ 1
      if G[j][i] == 'O':
        if last_free != j:
          G[last_free][i] = 'O'
          G[j][i] = '.'
        last_free += 1
  return G


def shift_south(G):
  for i in range(len(G[0])):
    last_free = len(G) - 1
    for j in range(len(G) - 1, -1, -1):
      if G[j][i] == '#':
        last_free = j - 1
      if G[j][i] == 'O':
        if last_free != j:
          G[last_free][i] = 'O'
          G[j][i] = '.'
        last_free -= 1
  return G

def shift_west(G):
  for i in range(len(G)):
    last_free = 0
    for j in range(len(G[0])):
      if G[i][j] == '#':
        last_free = j + 1
      if G[i][j] == 'O':
        if last_free != j:
          G[i][last_free] = 'O'
          G[i][j] = '.'
        last_free += 1
  return G

def shift_east(G):
  for i in range(len(G)):
    last_free = len(G[0]) - 1
    for j in range(len(G[0]) - 1, -1, -1):
      if G[i][j] == '#':
        last_free = j - 1
      if G[i][j] == 'O':
        if last_free != j:
          G[i][last_free] = 'O'
          G[i][j] = '.'
        last_free -= 1
  return G


shifts = [shift_north, shift_west, shift_south, shift_east]
def hash(G):
  return ''.join([''.join(l) for l in G])


def do_cycle(G):
  for i in range(4):
    G = shifts[i](G)
  return G

def find_loop(G):
  Gmem = {}
  for i in range(1000000000):
    G = do_cycle(G)
    if hash(G) in Gmem:
      return Gmem[hash(G)], i - Gmem[hash(G)]
    Gmem[hash(G)] = i


def load(G):
  lo = 0
  for (j, l) in enumerate(G):
    for c in l:
      if c == 'O':
        lo += len(G) - j
  return lo

first, cycle = find_loop(deepcopy(D))
needs = (1000000000 - first) % cycle + first

for i in range(needs):
  D = do_cycle(D)
print(load(D))
