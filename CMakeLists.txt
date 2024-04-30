cmake_minimum_required(VERSION 3.28 FATAL_ERROR)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

# Validate user settings sanity, environment setup, prerequisites.
# Depending on platform, set up SDK versions, deployment target versions, etc.
include(Prelude)

# Version information; to be set by the build environment
include(Version)

project(immerse_rt
        # Languages are enabled on demand, mostly depending on the host OS and specific settings.
        # Rust compilation is managed entirely by Cargo and is outside of CMake control.
        LANGUAGES NONE
        VERSION ${IRT_VERSION}
        DESCRIPTION "Immersive audio rendering engine, reliant on HRTF application and real-time listener position tracking"
        HOMEPAGE_URL https://github.com/integer-overflown/immerse-rt
)

add_subdirectory(src)
