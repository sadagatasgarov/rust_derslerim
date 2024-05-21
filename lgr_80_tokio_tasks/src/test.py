# def aSelf(X, n):
#     product = [0]*n
#     for i in range(n):
#         productend=1
#         for j in range(n):
#             if i==j:
#                 continue
#             productend = productend * X[j]
#         product[i] = productend
#     return product


# print(aSelf([2,4,5], 3))




def listVurma(arr):
    rr = []
    a=1
    
    for i in arr:
        arr.pop()
        a = a * i
        rr.append(a)
    return rr

print(listVurma([2,4,5]))