#include <signal.h>

void set_sigint_handler(void (*handler)(int)) {
    signal(SIGINT, handler);
}
