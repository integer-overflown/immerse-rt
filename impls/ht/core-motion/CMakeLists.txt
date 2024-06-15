corrosion_import_crate(
        MANIFEST_PATH Cargo.toml
        IMPORTED_CRATES BRIDGE_TARGET_NAME
)

set(BRIDGING_HEADER_FILE_NAME objc-bridging-header.h)
set(BRIDGING_HEADER_TEMPLATE_NAME ${BRIDGING_HEADER_FILE_NAME}.in)

set(GENERATED_DIR ${CMAKE_CURRENT_BINARY_DIR}/generated)

set(BRIDGING_HEADER_TEMPLATE_PATH "${CMAKE_CURRENT_LIST_DIR}/cmake/${BRIDGING_HEADER_TEMPLATE_NAME}")
set(BRIDGING_HEADER_FILE_PATH "${GENERATED_DIR}/${BRIDGING_HEADER_FILE_NAME}")

# region Inlined variables (see the file at BRIDGING_HEADER_NAME)
set(PATH_PREFIX ".")
set(TARGET_NAME ${BRIDGE_TARGET_NAME})
# endregion

configure_file(
        ${BRIDGING_HEADER_TEMPLATE_PATH}
        ${BRIDGING_HEADER_FILE_PATH}
        @ONLY
)

set(GENERATED_SWIFT
        ${GENERATED_DIR}/${BRIDGE_TARGET_NAME}/${BRIDGE_TARGET_NAME}.swift
        ${GENERATED_DIR}/SwiftBridgeCore.swift
)

set(GENERATED_HEADERS
        ${GENERATED_DIR}/${BRIDGE_TARGET_NAME}/${BRIDGE_TARGET_NAME}.h
        ${GENERATED_DIR}/SwiftBridgeCore.h
)

add_custom_target(irt_make_bridge
        COMMAND rs::swift_bridge parse-bridges --output ${GENERATED_DIR} --crate-name ${BRIDGE_TARGET_NAME} -f src/lib.rs
        WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
)

add_custom_command(
        OUTPUT
        ${GENERATED_SWIFT}
        ${GENERATED_HEADERS}
        DEPENDS src/lib.rs
        COMMAND irt_make_bridge
        COMMENT "Generating bridging files..."
        VERBATIM
)

add_library(irt_bridge INTERFACE)
target_sources(irt_bridge INTERFACE ${GENERATED_SWIFT})

# TODO(max-khm): figure out target location dynamically
# Current logic is a stub and will break in non-debug builds:
# we need to detect active cargo profile (or have a setting for this)
# and use the corresponding target dir.
find_library(BRIDGE_LIB NAMES ${BRIDGE_TARGET_NAME} PATHS "${CMAKE_CURRENT_LIST_DIR}/target/debug" REQUIRED)

target_compile_options(irt_bridge INTERFACE -import-objc-header ${BRIDGING_HEADER_FILE_PATH})
target_link_libraries(irt_bridge INTERFACE ${BRIDGE_LIB})

file(GENERATE
        OUTPUT ${CMAKE_CURRENT_LIST_DIR}/autogen/mod.rs
        INPUT ${CMAKE_CURRENT_LIST_DIR}/cmake/autogen/mod.rs.in
)

add_subdirectory(swift)

add_dependencies(${BRIDGE_TARGET_NAME} ht_core_motion)
