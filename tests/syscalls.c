void print(const char *s, int len) {

  register const char *a1 asm("a1") = s;
  register const char a2 asm("a2") = len;

  asm volatile("li a7, 64\n"
               "li a0, 1\n"
               "ecall\n"
               :
               : "r"(a1), "r"(a2)
               :);
}
