include_guard(GLOBAL)

include(ProjectInfo)

if (CMAKE_SOURCE_DIR STREQUAL CMAKE_BINARY_DIR)
    message(
            FATAL_ERROR
            "In-source builds are not supported"
    )
endif ()

if (CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
    include(${CMAKE_CURRENT_LIST_DIR}/preludes/MacOS.cmake)
else ()
    message(FATAL_ERROR "It appears that you're running on ${CMAKE_HOST_SYSTEM_NAME}, which is not currently supported.
Feel free to file an issue at ${IRT_REPO_URL} if you have a functionality proposal or a particular use-case you're interesting in seeing.")

endif ()
