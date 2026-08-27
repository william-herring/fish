!use math
// Can we skip the !
// seems like unnecessary special char

fun main() : void {
    // Integer
    int my_int = 5

    // Floating-point
    float pi = 3.14

    // Addition, subtraction
    print(my_int + 2) // "7"
    print(pi - 1.2) // "1.94"

    // Multiplication, division
    print(pi * my_int) // "15.7"
    print(my_int / pi) // "1.5923566879"

    // Exponents, logarithms (requires math)
    print(math.pow(pi, 2)) // "9.8596"
    print(math.log(math.e)) // "1" (base is e by default)
    print(math.log(100, 10)) // "2"
}

main()
