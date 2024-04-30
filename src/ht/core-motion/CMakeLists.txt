enable_language(Swift)

add_executable(
        ht_core_motion
        MACOSX_BUNDLE
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
)

target_compile_options(ht_core_motion PRIVATE -parse-as-library)
