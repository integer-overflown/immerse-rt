include_guard(GLOBAL)

if (CMAKE_SOURCE_DIR STREQUAL CMAKE_BINARY_DIR)
    message(
            FATAL_ERROR
            "In-source builds are not supported"
    )
endif ()

include(${CMAKE_CURRENT_LIST_DIR}/preludes/MacOS.cmake)
