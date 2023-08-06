#include <stdio.h>
#include <ctype.h>
#include <unistd.h>
#include <termios.h>

const char ALT_SCREEN_ESCAPE[] = "\e[?1049h";
const char ALT_SCREEN_RETURN_ESCAPE[] = "\e[?1049l";
const char CLEAR_SCREEN[] = "\033[2J";
const char GET_CURSOR_POS[] = "\033[6n";
const char RESET_CURSOR_POS[] = "\033[1;1H";

void switch_to_alt_screen() {
    printf(ALT_SCREEN_ESCAPE);
}

void return_from_alt_screen() {
    printf(ALT_SCREEN_RETURN_ESCAPE);
    fflush(stdout);
}

int clear_screen_with_width() {
    // Pretty much a clone of
    // https://stackoverflow.com/questions/74431114/get-terminal-size-using-ansi-escape-sequences
    struct termios old_settings, new_settings;

    char in[1024];
    int each;
    int ch;
    int rows;
    int cols;

    // Set some terminal settings
    tcgetattr(STDIN_FILENO, &old_settings);
    new_settings = old_settings;
    new_settings.c_lflag &= ~(ICANON | ECHO);
    new_settings.c_cc[VMIN] = 1;
    new_settings.c_cc[VTIME] = 0;
    tcsetattr(STDIN_FILENO, TCSANOW, &new_settings);

    printf(CLEAR_SCREEN);

    printf("\033[9999;9999H"); // Move the cursor really far away

    printf(GET_CURSOR_POS);
    while ((ch = getchar ()) != 'R') {
        if (EOF == ch) {
            break;
        }
        if (isprint (ch)) {
            if (each + 1 < 1024) {
                in[each] = ch;
                each++;
                in[each] = '\0';
            }
        }
    }

    printf("\033[1;1H"); // Reset the cursor position
    sscanf(in, "[%d;%d", &rows, &cols);
    fflush(stdout);

    // Revert to the old settings
    tcsetattr(STDIN_FILENO, TCSANOW, &old_settings);

    return cols;
}

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
