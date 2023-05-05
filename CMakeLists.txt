cmake_minimum_required(VERSION 3.23)

project(rusty_cmake CXX)

include(FetchContent)

set(CMAKE_EXPORT_COMPILE_COMMANDS true)
set(CMAKE_CXX_STANDARD 17)

# Fetch Needed tools
FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
if (CXXBRIDGE STREQUAL "CXXBRIDGE-NOTFOUND")
    message("Could not find cxxbridge, trying to install with `cargo install cxxbridge-cmd'")
    find_program(CARGO cargo PATHS "$ENV{HOME}/.cargo/bin/")
    if (CARGO STREQUAL "CARGO-NOTFOUND")
        message(FATAL_ERROR "Requires cargo available in path, install via rustup https://rustup.rs/")
    endif()
    execute_process(COMMAND ${CARGO} install cxxbridge-cmd)
    find_program(CXXBRIDGE cxxbridge PATHS "$ENV{HOME}/.cargo/bin/")
endif()

# Use tools to create cmake targets from rust
corrosion_import_crate(MANIFEST_PATH Cargo.toml)
corrosion_add_cxxbridge(rusty_lib CRATE reqwest_as_cpp MANIFEST_PATH reqwest_as_cpp FILES lib.rs)

add_executable(main_cpp main.cpp)
target_link_libraries(main_cpp PRIVATE rusty_lib)

