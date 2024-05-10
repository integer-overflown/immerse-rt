set(SWIFT_BRIDGE_CLI_NAME "swift-bridge-cli")
set(SWIFT_BRIDGE_CLI_VERSION "0.1.55")
set(SWIFT_BRIDGE_CLI_VERSIONED_NAME ${SWIFT_BRIDGE_CLI_NAME}@${SWIFT_BRIDGE_CLI_VERSION})
find_program(SWIFT_BRIDGE_CLI NAMES ${SWIFT_BRIDGE_CLI_NAME})

if (NOT SWIFT_BRIDGE_CLI)
    if (NOT IRT_AUTO_DOWNLOAD_SWIFT_BRIDGE)
        message(FATAL_ERROR "Cannot find ${SWIFT_BRIDGE_CLI_NAME} in PATH.
Please install it manually using \"cargo install\" or enable auto-download option")
    endif ()

    message(STATUS "${SWIFT_BRIDGE_CLI_NAME} not found - downloading")

    execute_process(
            COMMAND cargo install ${SWIFT_BRIDGE_CLI_VERSIONED_NAME}
            COMMAND_ECHO STDOUT
    )

    find_program(SWIFT_BRIDGE_CLI NAMES ${SWIFT_BRIDGE_CLI_NAME} REQUIRED)
endif ()

add_executable(rs::swift_bridge IMPORTED)
set_target_properties(rs::swift_bridge PROPERTIES IMPORTED_LOCATION ${SWIFT_BRIDGE_CLI})
