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