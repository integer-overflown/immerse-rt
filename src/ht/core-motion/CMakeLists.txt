enable_language(Swift)

add_executable(
        SwiftExample
        main.swift
)

find_library(FRAMEWORK_CORE_MOTION "CoreMotion" REQUIRED)

target_link_libraries(
        SwiftExample
        PRIVATE
        ${FRAMEWORK_CORE_MOTION}
)

target_compile_options(SwiftExample PRIVATE -parse-as-library)
