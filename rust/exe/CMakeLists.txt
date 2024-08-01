# Import your Rust targets - rust_ffi_exe
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

# Link C/C++ libraries with your Rust target
corrosion_link_libraries(rust_ffi_exe HelloFromCPPLib)

# Corrosion doesn't import the dependencies of the Rust target, so you have to do it manually
set(RUSTFLAGS "-L ${CMAKE_CURRENT_BINARY_DIR}/../lib -lrust_ffi_lib")
corrosion_set_env_vars(rust_ffi_exe "RUSTFLAGS=${RUSTFLAGS}")

# Rust test macro in order to integrate CTest with Rust tests
get_target_property(HelloFromCPPLibDir HelloFromCPPLib ARCHIVE_OUTPUT_DIRECTORY)
macro(add_rust_test target)
    add_test(NAME ${target}
        COMMAND cargo test ${target}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
    set_property (TEST ${target}
        PROPERTY ENVIRONMENT "RUSTFLAGS=-L ${HelloFromCPPLibDir} ${RUSTFLAGS}")
endmacro()

# Add your Rust tests (must be the same name as defined in main.rs)
add_rust_test(rusttest_get_string_from_cpp)
add_rust_test(rusttest_get_vector_from_cpp)
add_rust_test(rusttest_get_integer)