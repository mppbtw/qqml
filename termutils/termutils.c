#include <stdio.h>
#include <ctype.h>
#include <termios.h>

#define STDIN_FILENO 0
#define SIZE 1024
#define cursorjmp(cols, lines) printf("\033[%d;%dH", cols, lines);

const char ANSI_CLEAR[] = "\033[2J";
const char ANSI_EXIT_ALT_SCREEN[] = "\033[?1049l";
const char ANSI_ENTER_ALT_SCREEN[] = "\033[?1049h";
const char ANSI_GET_CURSOR_POS[] = "\033[6n";

struct TerminalSize clear_screen_with_termsize();

struct TerminalSize {
    int width;
    int height;
};

struct CursorPosition {
    int cols;
    int lines;
};
struct CursorPosition get_cursor_position() {
    struct termios original, changed;
    // Store the initial terminal settings
    tcgetattr(STDIN_FILENO, &original);
    changed = original;

    // Set the new setings
    changed.c_lflag &= ~(ICANON | ECHO);
    changed.c_cc[VMIN] = 1;
    changed.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &changed);

    // Put the cursor pos into stdin
    printf(ANSI_GET_CURSOR_POS);

    // Read from that buffer
    int i = 0;
    int ch = 0;
    int lines = 0;
    int cols = 0;
    char response[SIZE] = "";
    while ((ch = getchar()) != 'R') { // R terminates the response
        if (EOF == ch) {
            break;
        }
        if (isprint (ch)) {
            if (i + 1 < SIZE) {
                response[i] = ch;
                i++;
                response[i] = '\0';
            }
        }
    }
    // Get the rows and cols from the input
    sscanf(response, "[%d;%d", &lines, &cols);
    struct CursorPosition p = { cols, lines };
    fflush(stdout);
    return p;
}
int get_cursor_cols() {
    return get_cursor_position().cols;
}
int get_cursor_lines() {
    return get_cursor_position().lines;
}

char read_single_char() {
    struct termios old_settings, new_settings;
    tcgetattr(STDIN_FILENO, &old_settings);
    new_settings = old_settings;

    new_settings.c_lflag &= (~ICANON & ~ECHO);
    tcsetattr(STDIN_FILENO, TCSANOW, &new_settings);
    char c = getc(stdin);
    tcsetattr(STDIN_FILENO, TCSANOW, &old_settings);
    return c;
}

void enter_alt_screen() {
    printf(ANSI_ENTER_ALT_SCREEN);
    fflush(stdout);
}
void exit_alt_screen() {
    printf(ANSI_EXIT_ALT_SCREEN);
    fflush(stdout);
}

void hide_cursor() {
    printf("\e[?25l");
    fflush(stdout);
}
void show_cursor() {
    printf("\e[?25h");
    fflush(stdout);
}

int clear_screen_with_width() {
    return clear_screen_with_termsize().width;
}
int clear_screen_with_height() {
    return clear_screen_with_termsize().height;
}

struct TerminalSize clear_screen_with_termsize() {
    struct termios original, changed;

    // change terminal settings
    tcgetattr(STDIN_FILENO, &original);
    changed = original;
    changed.c_lflag &= ~(ICANON | ECHO);
    changed.c_cc[VMIN] = 1;
    changed.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &changed);

    printf(ANSI_CLEAR);

    // Move the cursor really far awway
    cursorjmp(999, 999);

    struct CursorPosition pos = get_cursor_position();

    // move to upper left corner
    cursorjmp(1, 1);

    // restore terminal settings
    tcsetattr(STDIN_FILENO, TCSANOW, &original);
    fflush(stdout);

    struct TerminalSize t = {pos.cols, pos.lines};
    return t;
}

void clear_screen() {
    printf(ANSI_CLEAR);
    cursorjmp(1, 1);
    fflush(stdout);
}

void close_stdin() {
    fclose(stdin);
}
