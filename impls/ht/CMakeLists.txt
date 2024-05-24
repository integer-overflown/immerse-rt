if (CMAKE_HOST_SYSTEM_NAME STREQUAL "Darwin")
    message(STATUS "Running on macOS - configuring CoreMotion implementation")
    add_subdirectory(core-motion)
endif ()
