find_program(CBINDGEN_COMMAND cbindgen REQUIRED)

set(HEADER_OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR}/bridge-include)

corrosion_import_crate(
        MANIFEST_PATH Cargo.toml
        CRATE_TYPES cdylib
)

add_custom_command(
        OUTPUT ${HEADER_OUTPUT_DIR}/bridge/irt.h
        COMMAND ${CBINDGEN_COMMAND} --output ${HEADER_OUTPUT_DIR}/bridge/irt.h
        WORKING_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}
        DEPENDS src/interop.rs
)

add_library(client_lib_bridge INTERFACE
        ${HEADER_OUTPUT_DIR}/bridge/irt.h
)

target_include_directories(client_lib_bridge INTERFACE ${HEADER_OUTPUT_DIR})
target_link_libraries(client_lib_bridge INTERFACE client_frontend_lib)
