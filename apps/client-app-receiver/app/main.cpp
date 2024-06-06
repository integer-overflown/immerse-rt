#include <format>
#include <stdexcept>

#include <QGuiApplication>
#include <QLoggingCategory>
#include <QQmlApplicationEngine>
#include <QQuickItem>
#include <QQuickWindow>
#include <QRunnable>
#include <QtConcurrent>

#include "bridge/irt.h"

enum ExitCode : std::int16_t {
    InitFailed = 1,
    CannotLoadQml = 2,
    FailedToConnect = 3
};

constexpr auto cServerUrl = "http://127.0.0.1:3000";

namespace utils {
template <typename Fn> class Defer {
  public:
    Defer(Fn &&fn) : fn_(std::forward<Fn>(fn)) {}

    Q_DISABLE_COPY_MOVE(Defer);

    ~Defer() noexcept(noexcept(fn_())) { fn_(); }

  private:
    Fn fn_;
};
} // namespace utils

namespace logging {
namespace {
Q_LOGGING_CATEGORY(app, "irt.app")
}
} // namespace logging

namespace app {
namespace {

class RequestFailed : public std::runtime_error {
  public:
    explicit RequestFailed(irt::RequestErrorCode e)
        : std::runtime_error(
              std::format("Request failed with {}", int(e)).c_str()) {}
};

class CreateStreamFailed : public std::runtime_error {
  public:
    explicit CreateStreamFailed(irt::CreateStreamErrorCode e)
        : std::runtime_error(
              std::format("Cannot create stream: {}", int(e)).c_str()) {}
};

class SetupStreamFailed : public std::runtime_error {
  public:
    SetupStreamFailed() : std::runtime_error("Failed to setup stream") {}
};

class StartStreamFailed : public std::runtime_error {
  public:
    StartStreamFailed() : std::runtime_error("Failed to start stream") {}
};

QFuture<irt::RequestResult> request_token(const char *serverUrl,
                                          irt::RoomOptions options) {
    return QtConcurrent::run([serverUrl, options] {
        return irt::request_token(serverUrl, options);
    });
}

} // namespace
} // namespace app

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

    auto rootObject = static_cast<QQuickWindow *>(engine.rootObjects().first());
    auto *videoItem = rootObject->findChild<QQuickItem *>("videoItem");

    if (!videoItem) {
        qCritical(logging::app()) << "Cannot find video item";
        return CannotLoadQml;
    }

    app::request_token(cServerUrl,
                       irt::RoomOptions{
                           .room_id = "room#1",
                           .identity = "sample-subscriber",
                           .name = nullptr,
                       })
        .then(qApp,
              [videoItem](irt::RequestResult result) {
                  utils::Defer _ =
                      std::bind_front(irt::free_request_result, result);

                  if (result.success) {
                      qDebug(logging::app()) << "Creating stream object";
                      return irt::create_stream(result.payload.value,
                                                videoItem);
                  }

                  throw app::RequestFailed(result.payload.error);
              })
        .then([](irt::CreateStreamResult result) {
            if (result.success) {
                qDebug(logging::app()) << "Unwrapping stream controller";
                return result.payload.value;
            }

            throw app::CreateStreamFailed(result.payload.error);
        })
        .then([videoItem](irt::StreamController *controller) {
            qDebug(logging::app()) << "Ready to start";

            QPromise<irt::StreamController *> p;
            auto future = p.future();

            videoItem->window()->scheduleRenderJob(
                QRunnable::create([p = std::move(p), controller]() mutable {
                    if (!irt::setup_stream(controller)) {
                        throw app::SetupStreamFailed{};
                    }

                    p.addResult(controller);
                    p.finish();
                }),
                QQuickWindow::BeforeSynchronizingStage);

            return future;
        })
        .unwrap()
        .then([](irt::StreamController *controller) {
            if (!irt::start_stream(controller)) {
                throw app::StartStreamFailed{};
            }
        })
        .onFailed([](const std::exception &e) {
            qCritical(logging::app()) << "Failed with exception:" << e.what();
            QCoreApplication::exit(FailedToConnect);
        });

    return QCoreApplication::exec();
}
