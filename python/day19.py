insr, valr = open(0).read().strip().split("\n\n")
ins, val = {}, []
for i in insr.split("\n"):
  rule, other = i.split('{')
  pos = []
  parts = other[:-1].split(',')
  default = parts[-1]
  for j in parts[:-1]:
    a, b = j.split(':')
    if '<' in a:
      c, d = a.split('<')
      pos.append((c, '<', int(d), b))
    else:
      c, d = a.split('>')
      pos.append((c, '>', int(d), b))
  ins[rule] = (pos, default)

for i in valr.split("\n"):
  parts = i[1:-1].split(',')
  p = {}
  for j in parts:
    a, b = j.split('=')
    p[a] = int(b)
  val.append(p)

def process(v):
  curr = ins['in']
  while True:
    n = None
    for a in curr[0]:
      if a[1] == '<' and v[a[0]] < a[2]:
        n = a[3]
        break
      elif a[1] == '>' and v[a[0]] > a[2]:
        n = a[3]
        break
    if n is None:
      n = curr[1]
    if n == 'A':
      return True
    if n == 'R':
      return False
    curr = ins[n]
r = 0
for v in val:
  if process(v):
    r += sum(v.values())

# I cheated on this, this comes from hyper-neutrino solution
def part2(ranges, name = 'in'):
  if name == 'A':
    prod = 1
    for l, h in ranges.values():
      prod *= h - l + 1
    return prod
  if name == 'R':
    return 0
  pos, default = ins[name]

  total = 0
  for cond, op, n, target in pos:
    l, h = ranges[cond]
    if op == '<':
      tr = (l, n - 1)
      fr = (n, h)
    else:
      tr = (n + 1, h)
      fr = (l, n)
    if tr[0] <= tr[1]:
      nr = dict(ranges)
      nr[cond] = tr
      total += part2(nr, target)
    if fr[0] <= fr[1]:
      ranges = dict(ranges)
      ranges[cond] = fr
    else:
      break
  else:
    total += part2(ranges, default)
  
  return total
      
print(r)
print(part2({key: (1, 4000) for key in 'xmas'}))
