
import time

def fib(n):
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fib(n-1) + fib(n-2)


def fib_seq(n):
    a=0
    b=1
    for i in range(n):
        a,b = b, a+b
    
    return a

def main():
    number = input("Enter a number: ")
    try:
        number = int(number)
    except ValueError:
        print("Please enter a valid number")
        return main()

    start = time.time()
    result = fib_seq(number)
    end = time.time()

    print(f"Fibinacci of {number} is {result} and took {end-start}")
    main()
main()