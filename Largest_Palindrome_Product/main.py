def palindrome():
    maxi = []
    for i in range(100,999):
        for j in range(100,999):
            x = str(i*j)
            if x == x[::-1]:
                maxi.append(int(x))

    print(max(maxi))

palindrome()