include(${CMAKE_CURRENT_LIST_DIR}/deps/Generic.cmake)

if (CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
    include(${CMAKE_CURRENT_LIST_DIR}/deps/MacOS.cmake)
endif ()
