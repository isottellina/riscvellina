int fib(int a, int b) {
    return a + b;
}

int main() {
    int a = 0, b = 1;

    while (b < 50) {
        int c = b;
        b = fib(a, b);
        a = c;
        asm("mv t0, %0" :: "r"(b));
    }

    asm("mv t0, %0" :: "r"(b));
    asm("nop");
    return 0;
}