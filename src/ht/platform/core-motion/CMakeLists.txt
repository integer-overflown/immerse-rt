enable_language(Swift)

add_library(
        ht_core_motion
        STATIC
        main.swift
)

set_target_properties(ht_core_motion
        PROPERTIES
        MACOSX_BUNDLE_INFO_PLIST ${CMAKE_CURRENT_LIST_DIR}/config/Info.plist.in
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
