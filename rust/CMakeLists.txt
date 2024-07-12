include(FetchContent)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG        v0.5.0
)
# Set any global configuration variables such as `Rust_TOOLCHAIN` before this line!
FetchContent_MakeAvailable(Corrosion)

# Import your Rust targets
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

# Link C/C++ libraries with your Rust target
corrosion_link_libraries(rust_ffi HelloFromCPP)

# Rust test macro in order to integrate CTest with Rust tests
get_target_property(HelloFromCPPLibDir HelloFromCPP ARCHIVE_OUTPUT_DIRECTORY)
message(STATUS "HelloFromCPPLibDir: ${HelloFromCPPLibDir}")
macro(add_rust_test target)
    add_test(NAME ${target}
        COMMAND cargo test ${target}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
    set_property (TEST ${target}
        PROPERTY ENVIRONMENT "RUSTFLAGS=-L ${HelloFromCPPLibDir}")
endmacro()

# Add your Rust tests (must be the same name as defined in main.rs)
add_rust_test(test_get_string_from_cpp)
add_rust_test(test_get_integer)