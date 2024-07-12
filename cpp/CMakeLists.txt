project(cpp_lib)

set(CMAKE_CXX_STANDARD 11)

add_library(HelloFromCPP STATIC main.cpp)
set_target_properties(HelloFromCPP PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")