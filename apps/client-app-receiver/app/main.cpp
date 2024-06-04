#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickItem>
#include <QQuickWindow>
#include <QtConcurrent>

#include <type_traits>

#include "bridge/irt.h"

enum ExitCode : std::int16_t { InitFailed = 1, CannotLoadQml = 2 };

constexpr auto cServerUrl = "http://127.0.0.1:3000";

namespace {
void request_token(const char *serverUrl, irt::RoomOptions options,
                   std::invocable<const irt::RequestResult &> auto visitor) {
    QtConcurrent::run([serverUrl, options] {
        return irt::request_token(serverUrl, options);
    }).then([visitor = std::move(visitor)](irt::RequestResult result) {
        visitor(result);
        irt::free_result(result);
    });
}
} // namespace

int main(int argc, char *argv[]) {
    if (!irt::init()) {
        return InitFailed;
    }

    QGuiApplication app(argc, argv);
    QQuickWindow::setGraphicsApi(QSGRendererInterface::OpenGL);

    QQmlApplicationEngine engine;

    QObject::connect(
        &engine, &QQmlApplicationEngine::objectCreated, qApp, [&engine] {
            if (engine.rootObjects().isEmpty()) {
                QCoreApplication::exit(CannotLoadQml);
            }

            auto rootObject =
                static_cast<QQuickWindow *>(engine.rootObjects().first());
            auto *videoItem = rootObject->findChild<QQuickItem *>("videoItem");

            Q_ASSERT(videoItem);
        });

    engine.load(QUrl{u"qrc:/main.qml"_qs});

    return QGuiApplication::exec();
}
