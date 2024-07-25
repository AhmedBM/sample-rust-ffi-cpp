#include <string>
#include <cstring>
#include <vector>

extern "C" {
    const char* get_string();

    int32_t get_integer();

    const std::vector<int>* get_vector();

    void get_vector_data(const std::vector<int>& vec, const int** data, size_t* size);

    void send_vector_to_cpp(void* data, size_t size);   // This function is defined in the Rust library
    std::vector<int> get_vector_from_rust(size_t len);

    void deallocate_res(void* res);
}