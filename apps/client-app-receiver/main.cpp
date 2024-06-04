#include <QGuiApplication>
#include <QQmlApplicationEngine>
#include <QQuickWindow>
#include <QtConcurrent>

#include "bridge/irt.h"

enum ExitCode : std::int16_t { InitFailed = 1, CannotLoadQml = 2 };

constexpr auto cServerUrl = "http://127.0.0.1:3000";

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

    QtConcurrent::run([] {
        return irt::request_token(cServerUrl, irt::RoomOptions{
                                                  .room_id = "room#1",
                                                  .identity = "producer",
                                                  .name = nullptr,
                                              });
    }).then([](irt::RequestResult result) {
        if (result.success) {
            qDebug() << "Success";
            qDebug() << "Token:" << result.payload.token;
        } else {
            qDebug() << "Failure";
            qDebug() << "Error:" << int(result.payload.error);
        }

        irt::free_result(result);
    });

    return QGuiApplication::exec();
}
