from copy import deepcopy


def indices(dim):
    r = []
    if all(map(lambda x: x > 0, dim)):
        r.append([0 for i in range(len(dim))])
        while True:
            s = r[-1]
            j = 0
            while True:
                if j == len(dim):
                    return sorted(r)
                elif s[j] < dim[j] - 1:
                    s[j] += 1
                    break
                else:
                    s[j] = 0
                    j += 1
            r.append(list(s))
    return []
