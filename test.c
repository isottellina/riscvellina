int fib(int a, int b) {
    return a + b;
}

int multiply(int a, int b) {
    return a % b;
}

int main() {
    int a = 0, b = 1;

    while (b < 250) {
        int c = b;
        b = fib(a, b);
        a = c;
        asm("mv t0, %0" :: "r"(b));
    }

    asm("mv t0, %0" :: "r"(a));
    int test = multiply(a, 15);
    asm("mv t1, %0" :: "r"(test));
    asm("nop");
    return 0;
}