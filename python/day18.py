D = [(a[0], int(a[1]), a[2][2:-1]) for a in (a.split() for a in open(0).read().strip().split('\n'))]

DX = {'R': 1, 'L': -1, 'U': 0, 'D': 0}
DY = {'R': 0, 'L': 0, 'U': -1, 'D': 1}

for part2 in [False, True]:
  pos = (0, 0)
  border = 0
  points = [pos]
  for c in D:
    dist = int(c[2][:5], 16) if part2 else c[1]
    _dir = 'RDLU'[int(c[2][5])] if part2 else c[0]
    border += dist
    pos = (pos[0] + DX[_dir] * dist, pos[1] + DY[_dir] * dist)
    points.append(pos)
  
  s = abs(sum(points[i][0] * (points[i - 1][1] - points[(i + 1) %len(points)][1]) for i in range(len(points)))) // 2
  print(s + border // 2 + 1)
