project(cpp_exe)

set(CMAKE_CXX_STANDARD 11)

find_package(GTest REQUIRED)
add_executable(HelloFromCPPExe main.cpp)
target_include_directories(HelloFromCPPExe PRIVATE ${GTEST_INCLUDE_DIRS} ${CPP_FFI_LIB_DIR})
target_link_libraries(HelloFromCPPExe rust_ffi_lib HelloFromCPPLib ${GTEST_LIBRARIES})

enable_testing()
gtest_discover_tests(HelloFromCPPExe)