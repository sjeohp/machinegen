from functools import reduce
from operator import mul

def fill_div(i, d):
	if len(d) < 1: 
		return 0
	elif len(d) == 1:
		return int(i / d[-1])
	else:
		return int(fill_div(i, d[:-1]) / d[-1])

def fill_mod(i, d):
	if len(d) < 1: 
		return []
	elif len(d) == 1:
		return [i % d[-1]]
	else:
		return fill_mod(i, d[:-1]) + [fill_div(i, d[:-1]) % d[-1]]

def fill(i, d, r):
	if i == reduce(mul, d, 1):
		return r
	else:
		return r + fill(i+1, d, [fill_mod(i, d)])

# example usage
#nx = 3
#ny = 4
#nz = 5
#res = fill(0, [nx, ny, nz], [])
#print('\n'.join('{}:\t {}'.format(*k) for k in enumerate(res)))
