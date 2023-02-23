import numpy as np

s = 31
M = 20000
m = 20
E = 2

for j in [int(i) for i in np.arange(0,1,1/s)**E * (M - m) + m]:
	print('  - fc: ' + str(j))
