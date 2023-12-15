A = open(0).read().split(",")
r = 0
boxes = [[] for i in range(256)]

def hash(s):
  h = 0
  for c in s:
    h += ord(c)
    h *= 17
    h %= 256
  return h

for a in A:
  r += hash(a)
  if '-' in a:
    boxID = hash(a[:-1])
    boxes[boxID] = [x for x in boxes[boxID] if x[0] != a[:-1]]
  else:
    boxID = hash(a[:-2])
    for i, x in enumerate(boxes[boxID]):
      if x[0] == a[:-2]:
        boxes[boxID][i] = (a[:-2], int(a[-1]))
        break
    else:
      boxes[boxID].append((a[:-2], int(a[-1])))
print(r)

r = 0
for i, b in enumerate(boxes):
  for j, f in enumerate(b):
    r += (i+1) * (j+1) * f[1]
print(r)
