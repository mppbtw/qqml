#include <stdio.h>
#include <ctype.h>
#include <unistd.h>
#include <termios.h>

#define SIZE 1024

struct TerminalSize clear_screen_with_termsize();

struct TerminalSize {
    int width;
    int height;
};

void enter_alt_screen() {
    printf("\033[?1049h");
    fflush(stdout);
}
void exit_alt_screen() {
    printf("\033[?1049l");
    fflush(stdout);
}

int clear_screen_with_width() {
    return clear_screen_with_termsize().width;
}
int clear_screen_with_heigt() {
    return clear_screen_with_termsize().height;
}

struct TerminalSize clear_screen_with_termsize() {
    char in[SIZE] = "";
    int each = 0;
    int ch = 0;
    int rows = 0;
    int cols = 0;
    struct termios original, changed;

    // change terminal settings
    tcgetattr(STDIN_FILENO, &original);
    changed = original;
    changed.c_lflag &= ~(ICANON | ECHO);
    changed.c_cc[VMIN] = 1;
    changed.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &changed);

    printf ("\033[2J"); //clear screen

    printf ("\033[9999;9999H"); // cursor should move as far as it can

    printf("\033[6n"); // ask for cursor position
    while (( ch = getchar()) != 'R') { // R terminates the response
        if (EOF == ch) {
            break;
        }
        if (isprint (ch)) {
            if (each + 1 < SIZE) {
                in[each] = ch;
                each++;
                in[each] = '\0';
            }
        }
    }

    printf("\033[1;1H"); // move to upper left corner
    if ( 2 == sscanf ( in, "[%d;%d", &rows, &cols)) {
    }

    // restore terminal settings
    tcsetattr( STDIN_FILENO, TCSANOW, &original);
    fflush(stdout);

    struct TerminalSize t = {cols, rows};
    return t;
}
