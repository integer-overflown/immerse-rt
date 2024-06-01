#include <QGuiApplication>
#include <QQmlApplicationEngine>

enum ExitCode : std::int16_t { FailedToLoadQml = -1 };

int main(int argc, char *argv[]) {
    QGuiApplication app(argc, argv);

    QQmlApplicationEngine engine;
    QObject::connect(&engine, &QQmlApplicationEngine::objectCreated, qApp,
                     [&engine] {
                         if (engine.rootObjects().isEmpty()) {
                             QCoreApplication::exit(FailedToLoadQml);
                         }
                     });

    engine.load(QUrl{u"qrc:/main.qml"_qs});

    return QGuiApplication::exec();
}
