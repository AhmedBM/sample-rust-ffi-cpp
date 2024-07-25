#include "lib.h"

const char *get_string()
{
    // Allocate memory on the heap for the string so it can be returned to the caller
    char *str = new char[50];
    // Copy a string into the allocated memory
    std::strcpy(str, "Hello from C++!");
    return str;
}

int32_t get_integer()
{
    return 7;
}

const std::vector<int> *get_vector()
{
    return new std::vector<int>{1, 2, 3, 4, 5};
}

void get_vector_data(const std::vector<int> &vec, const int **data, size_t *size)
{
    *data = vec.data();
    *size = vec.size();
}

std::vector<int> get_vector_from_rust(size_t len) {
    std::vector<int> vec(len);
    send_vector_to_cpp(vec.data(), vec.size());
    return vec;
}

void deallocate_res(void *res)
{
    delete[] (char *)res;
}