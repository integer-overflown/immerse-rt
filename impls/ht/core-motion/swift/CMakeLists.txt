enable_language(Swift)

add_library(
        ht_core_motion
        STATIC
        lib.swift
)

find_library(FRAMEWORK_CORE_MOTION "CoreMotion" REQUIRED)

target_link_libraries(
        ht_core_motion
        PRIVATE
        ${FRAMEWORK_CORE_MOTION}
        irt_bridge
)

add_dependencies(ht_core_motion irt_make_bridge)

target_compile_options(ht_core_motion PRIVATE -parse-as-library)
