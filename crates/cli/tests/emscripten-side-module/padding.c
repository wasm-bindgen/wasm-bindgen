#include <emscripten/emscripten.h>

static volatile int data[1024] = {7};
static int padding(int value) { return value + 1; }
static int (*volatile functions[])(int) = {padding};

EMSCRIPTEN_KEEPALIVE int use_padding(int value) {
    return functions[0](value) + data[value & 1023];
}
