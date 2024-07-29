def fibo_even():
    start = [1,2]
    i = 1

    while start[i] < 4000000:
        if (start[i] + start[i-1])< 4000000:
            start.append(start[i]+start[i-1])
        else:
            return start
        i += 1

       
        
sum = 0

if __name__ == "__main__":
    all = fibo_even()
    for i in range(len(fibo_even())):
        if all[i] % 2 == 0:
            sum = sum + all[i]
    print(sum)
