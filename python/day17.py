import heapq

D = [[b for b in a] for a in open(0).read().strip().split("\n")]
DX=[0, 1, 0, -1]
DY=[-1, 0, 1, 0]

for part2 in [False, True]:
  q = [(0, (0, 0), -1, -1)]
  visited = {}
  while q:
    s, p, d, l = heapq.heappop(q)
    if (p, d, l) in visited:
      continue
    visited[(p, d, l)] = s
    for i in range(4):
      if (i+2)%4 == d:
        continue
      nl = l+1 if i==d else 1
      nx, ny = p[0] + DX[i], p[1] + DY[i]
      if part2:
        if nl > 10 or (i != d and l < 4 and l != -1):
          continue
        if nx == len(D[0])-1 and ny == len(D)-1 and nl < 4  :
          continue
      else:
        if nl > 3:
          continue
      if nx < 0 or ny < 0 or nx >= len(D[0]) or ny >= len(D):
        continue
      heapq.heappush(q, (s+int(D[ny][nx]), (nx, ny), i, nl))

  print(min(filter(lambda x: x[0][0] == (len(D[0])-1, len(D)-1), visited.items()), key=lambda x: x[1])[1])
