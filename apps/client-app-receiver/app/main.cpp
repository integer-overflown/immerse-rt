#include <QGuiApplication>
#include <QLoggingCategory>
#include <QQmlApplicationEngine>
#include <QQuickWindow>

#include "bridge/irt.h"

enum ExitCode : std::int16_t {
    InitFailed = 1,
    CannotLoadQml = 2,
    FailedToConnect = 3
};

namespace logging {
namespace {
Q_LOGGING_CATEGORY(app, "irt.app")
}
} // namespace logging

int main(int argc, char *argv[]) {
    if (!irt::init()) {
        return InitFailed;
    }

    QGuiApplication app(argc, argv);
    QQuickWindow::setGraphicsApi(QSGRendererInterface::OpenGL);

    QQmlApplicationEngine engine;

    engine.load(u"qrc:/main.qml"_qs);

    if (engine.rootObjects().isEmpty()) {
        return CannotLoadQml;
    }

    return QCoreApplication::exec();
}
