corrosion_import_crate(
        MANIFEST_PATH Cargo.toml
        IMPORTED_CRATES BRIDGE_TARGET_NAME
)

add_library(irt_bridge INTERFACE)

target_sources(irt_bridge INTERFACE
        generated/bridge/bridge.swift
        generated/SwiftBridgeCore.swift
)

set(BRIDGING_HEADER_FILE_NAME objc-bridging-header.h)
set(BRIDGING_HEADER_TEMPLATE_NAME ${BRIDGING_HEADER_FILE_NAME}.in)

set(BRIDGING_HEADER_TEMPLATE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake/${BRIDGING_HEADER_TEMPLATE_NAME}")
set(BRIDGING_HEADER_FILE_PATH "${CMAKE_CURRENT_LIST_DIR}/generated/${BRIDGING_HEADER_FILE_NAME}")

# region Inlined variables (see the file at BRIDGING_HEADER_NAME)
set(PATH_PREFIX ".")
set(TARGET_NAME ${BRIDGE_TARGET_NAME})
# endregion

configure_file(
        ${BRIDGING_HEADER_TEMPLATE_PATH}
        ${BRIDGING_HEADER_FILE_PATH}
        @ONLY
)

# TODO(max-khm): figure out target location dynamically
# Current logic is a stub and will break in non-debug builds:
# we need to detect active cargo profile (or have a setting for this)
# and use the corresponding target dir.
target_compile_options(irt_bridge INTERFACE
        -import-objc-header ${BRIDGING_HEADER_FILE_PATH}
        -L ${CMAKE_CURRENT_LIST_DIR}/target/debug # TODO: try to use find_library
        -l${BRIDGE_TARGET_NAME}
)

add_custom_target(irt_make_bridge
        COMMAND rs::swift_bridge parse-bridges --output generated ${BRIDGE_TARGET_NAME} src/lib.rs
        WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)

add_custom_command(
        OUTPUT
        generated/${BRIDGE_TARGET_NAME}/${BRIDGE_TARGET_NAME}.swift
        generated/${BRIDGE_TARGET_NAME}/${BRIDGE_TARGET_NAME}.h
        generated/SwiftBridgeCore.swift
        generated/SwiftBridgeCore.h
        DEPENDS src/lib.rs
        COMMAND irt_make_bridge
        COMMENT "Generating bridging files..."
        VERBATIM
)

add_subdirectory(swift)
