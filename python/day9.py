def f(l):
 l,r=list(map(int,l.split())),0
 while l:l,r=[s-t for t,s in zip(l,l[1:])],r+l[-1]
 return r
print(sum(map(f,open(0).readlines())))
