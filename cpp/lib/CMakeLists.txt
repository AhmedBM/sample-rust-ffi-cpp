project(cpp_lib)

set(CMAKE_CXX_STANDARD 11)

add_library(HelloFromCPPLib STATIC lib.cpp)
set_target_properties(HelloFromCPPLib PROPERTIES ARCHIVE_OUTPUT_DIRECTORY "${CMAKE_BINARY_DIR}/lib")