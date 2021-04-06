int add_two(int a, int b) {
    return a + b;
}

int main() {
    int a = 2, b = 3;
    int c = add_two(a, b);

    asm("mv t0, %0" :: "r"(c));
    asm("nop");
    return 0;
}