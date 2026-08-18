int puts(const char *);

int main() {
  puts("Hello World!\n");
  asm volatile("nop");
  return 0;
}
