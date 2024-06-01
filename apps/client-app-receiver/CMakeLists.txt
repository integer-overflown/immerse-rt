set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

enable_language(CXX)

find_package(Qt6 COMPONENTS Core Qml Quick REQUIRED)
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

add_executable(
        client-app-receiver
        MACOSX_BUNDLE
        ${HEADER_OUTPUT_DIR}/bridge/irt.h
        main.cpp
        res/qml.qrc
)

target_link_libraries(client-app-receiver PRIVATE Qt::Core Qt::Qml Qt::Quick client_frontend_lib)
target_include_directories(client-app-receiver PRIVATE ${HEADER_OUTPUT_DIR})