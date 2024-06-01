#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickWindow>

#include "bridge/irt.h"

enum ExitCode : std::int16_t { InitFailed = 1, CannotLoadQml = 2 };

int main(int argc, char *argv[]) {
    if (!irt::init()) {
        return InitFailed;
    }

    QGuiApplication app(argc, argv);
    QQuickWindow::setGraphicsApi(QSGRendererInterface::OpenGL);

    QQmlApplicationEngine engine;

    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated, qApp,
                     [&engine] {
                         if (engine.rootObjects().isEmpty()) {
                             QCoreApplication::exit(CannotLoadQml);
                         }
                     });

    engine.load(QUrl{u"qrc:/main.qml"_qs});

    return QGuiApplication::exec();
}
