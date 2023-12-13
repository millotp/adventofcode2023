import math
from copy import deepcopy
patterns = open(0).read().split("\n\n")

def has_sym(p):
  for i in range(len(p)-1):
    if p[i] == p[i+1]:
      has_sy = True
      # expand
      for j in range(len(p)):
        if i-j<0 or i+1+j>=len(p):
          break
        if not p[i-j] == p[i+1+j]:
          has_sy = False
      if has_sy:
        return i+1
  return 0

def rev_sym(p):
  for i in range(len(p)-2,-1,-1):
    if p[i] == p[i+1]:
      has_sy = True
      # expand
      for j in range(len(p)):
        if i-j<0 or i+1+j>=len(p):
          break
        if not p[i-j] == p[i+1+j]:
          has_sy = False
      if has_sy:
        return i+1
  return 0

r = 0
for p in patterns:
  p = [a for a in p.split('\n') if a != '']
  p_rot = list(zip(*p))
  r += has_sym(p) * 100 + has_sym(p_rot)
print(r)

def find_defect(p):
  s = has_sym(p)*100
  if s == 0:
    s = has_sym(list(zip(*p)))
  for j in range(len(p)):
    for i in range(len(p[0])):
      cp = deepcopy(p)
      if p[j][i] == '.':
        cp[j][i] = '#'
      else:
        cp[j][i] = '.'

      ns = has_sym(cp)*100
      if ns != 0 and ns != s:
        return ns
      ns = has_sym(list(zip(*cp)))
      if ns != 0 and ns != s:
        return ns
      ns = rev_sym(cp)*100
      if ns != 0 and ns != s:
        return ns
      ns = rev_sym(list(zip(*cp)))
      if ns != 0 and ns != s:
        return ns
      

  return 0

# Part 2
r = 0
for p in patterns:
  p = [list(a) for a in p.split('\n') if a != '']
  s = find_defect(p)
  if s == 0:
    print("no defect found")
  r += s
print(r)
