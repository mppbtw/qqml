#include <stdio.h>
#include <unistd.h>
#include <termios.h>

char read_single_char() {
    struct termios old_settings, new_settings;

    tcgetattr(STDIN_FILENO, &old_settings);
    new_settings = old_settings;
    new_settings.c_lflag &= (~ICANON & ~ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &new_settings);

    char c = getc(stdin);

    tcsetattr(STDIN_FILENO, TCSANOW, &old_settings);

    if (c == EOF || c == 0) {
        return 0;
    }
    return c;
}
