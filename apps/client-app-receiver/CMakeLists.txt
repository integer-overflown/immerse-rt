set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

enable_language(CXX)

find_package(Qt6 COMPONENTS Core Qml Quick Concurrent REQUIRED)

add_executable(
        client-app-receiver
        MACOSX_BUNDLE
        app/main.cpp
        res/qml.qrc
)

add_subdirectory(lib)

target_link_libraries(client-app-receiver PRIVATE
        Qt::Core
        Qt::Qml
        Qt::Quick
        Qt::Concurrent
        client_lib_bridge
)