initial = open(0).read().split("\n")

DX = [0, 1, 0, -1]
DY = [-1, 0, 1, 0]

D = [[(a, False, 0, False) for a in l] for l in initial if len(l) > 0]
laserPos = set()

def setup(x, y, d):
  global D, laserPos
  D = [[(a, False, 0, False) for a in l] for l in initial if len(l) > 0]
  D[y][x] = (D[y][x][0], True, d, True)

  laserPos = set()
  laserPos.add((x, y))

def setLaser(D, x, y, d):
  if 0 <= x < len(D[0]) and 0 <= y < len(D):
    D[y][x] = (D[y][x][0], True, d, True)
    laserPos.add((x, y))

def step(D):
  for i, j in list(laserPos):
    c, _, d, _ = D[j][i]
    D[j][i] = (D[j][i][0], False, 0, True)
    laserPos.remove((i, j))
    nx, ny = i + DX[d], j + DY[d]
    if c == '.':
      setLaser(D, nx, ny, d)
    elif c == '/':
      d = [1, 0, 3, 2][d]
      nx, ny = i + DX[d], j + DY[d]
      setLaser(D, nx, ny, d)
    elif c == '\\':
      d = [3, 2, 1, 0][d]
      nx, ny = i + DX[d], j + DY[d]
      setLaser(D, nx, ny, d)
    elif c == '|':
      if d == 0 or d == 2:
        setLaser(D, nx, ny, d)
      else:
        setLaser(D, i, j - 1, 0)
        setLaser(D, i, j + 1, 2)
    elif c == '-':
      if d == 1 or d == 3:
        setLaser(D, nx, ny, d)
      else:
        setLaser(D, i - 1, j, 3)
        setLaser(D, i + 1, j, 1)

def count(D):
  r = 0
  for l in D:
    for _, _, _, e in l:
      if e:
        r+=1
  return r

edge = len(D)
maxR = 0
for k in range(edge):
  print(k)
  setup(0, k, 1)
  for i in range(edge*edge):
    step(D)

  r = count(D)
  if r > maxR:
    maxR = r
  
  setup(edge-1, k, 3)
  for i in range(edge*edge):
    step(D)

  r = count(D)
  if r > maxR:
    maxR = r

  setup(k, 0, 2)
  for i in range(edge*edge):
    step(D)

  r = count(D)
  if r > maxR:
    maxR = r

  setup(k, edge-1, 0)
  for i in range(edge*edge):
    step(D)

  r = count(D)
  if r > maxR:
    maxR = r

print(maxR)
