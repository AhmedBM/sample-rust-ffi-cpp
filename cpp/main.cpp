#include <string>
#include <cstring>

extern "C" {
    const char* get_string() {
        // Allocate memory on the heap for the string so it can be returned to the caller
        char* str = new char[50];
        // Copy a string into the allocated memory
        std::strcpy(str, "Hello from C++!");
        return str;
    }

    int32_t get_integer() {
        return 7;
    }

    void deallocate_string(char* str) {
        delete[] str;
    }
}