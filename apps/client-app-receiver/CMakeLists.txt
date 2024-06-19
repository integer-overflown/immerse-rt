set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

enable_language(CXX)

find_package(Qt6 COMPONENTS Core Qml Quick Concurrent REQUIRED)

qt_add_executable(
        client-app-receiver
        MACOSX_BUNDLE
        app/main.cpp
        res/qml.qrc
        app/qml_utils.h
        app/app_instance.h
        app/app_instance.cpp
        app/publisher_controller.h
        app/subscriber_controller.h
        app/publisher_controller.cpp
        app/subscriber_controller.cpp
        app/constants.h
        app/utils.h
        app/exceptions.h
)

set_target_properties(client-app-receiver
        PROPERTIES
        MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/config/Info.plist.in
)

add_subdirectory(lib)

target_link_libraries(client-app-receiver PRIVATE
        Qt::Core
        Qt::Qml
        Qt::Quick
        Qt::Concurrent
        client_lib_bridge
)

qt_add_qml_module(client-app-receiver
        URI com.github.immerse_rt
        SOURCES
        app/qml_utils.h
        app/app_instance.h
        app/app_instance.cpp
        RESOURCE_PREFIX /
        VERSION 1.0
)

target_include_directories(client-app-receiver PRIVATE app)