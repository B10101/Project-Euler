def palindrome():
    for i in range(100,999):
        for j in range(100,999):
            x = str(i*j)
            if x == x[::-1]:

                print(x)

palindrome()