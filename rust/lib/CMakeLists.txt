# Import your Rust targets
corrosion_import_crate(MANIFEST_PATH Cargo.toml)

# Rust test macro in order to integrate CTest with Rust tests
macro(add_rust_test target)
    add_test(NAME ${target}
        COMMAND cargo test ${target}
        WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR})
endmacro()

# Add your Rust tests (must be the same name as defined in main.rs)
add_rust_test(rusttest_lib_test_it_works)