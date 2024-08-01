project(cpp_lib)

set(CMAKE_CXX_STANDARD 11)

add_library(HelloFromCPPLib STATIC lib.cpp)
# Needed so CTest can find the needed RUSTFLAGS to compile the Rust tests
set_target_properties(HelloFromCPPLib PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")