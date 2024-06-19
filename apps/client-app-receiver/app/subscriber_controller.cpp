#include "subscriber_controller.h"

#include <format>
#include <stdexcept>

#include <QQuickItem>
#include <QQuickWindow>
#include <QRunnable>
#include <QtConcurrent>

#include "bridge/irt.h"
#include "constants.h"
#include "exceptions.h"
#include "utils.h"

namespace app {

namespace {

namespace logging {
Q_LOGGING_CATEGORY(sub, "app.subscriber")
}

} // namespace

void SubscriberController::connectToStream(QQuickItem *videoSink) {
    status_.setValue(RequestingToken);

    app::utils::request_token(
        constants::serverUrl,
        irt::RoomOptions{.room_id = "room#1",
                         .identity = "sample-subscriber",
                         .name = nullptr,
                         .role = irt::PeerRole::Subscriber})
        .then(this,
              [this, videoSink](irt::RequestResult result) {
                  utils::Defer _ =
                      std::bind_front(irt::free_request_result, result);

                  status_.setValue(StartingStream);

                  QFile hrir(":/IRC_1002_C.bin");

                  if (!hrir.open(QIODevice::ReadOnly)) {
                      throw app::FailedToReadHRIR{};
                  }

                  auto bytes = hrir.readAll();

                  auto buf = irt::MemoryBuffer{
                      .data = reinterpret_cast<std::uint8_t *>(bytes.data()),
                      .len = static_cast<std::uintptr_t>(bytes.length())};

                  if (result.success) {
                      qDebug(logging::sub()) << "Creating stream object";
                      return irt::create_subscriber_stream(result.payload.value,
                                                           videoSink, buf);
                  }

                  throw app::RequestFailed(result.payload.error);
              })
        .then([](irt::CreateSubscriberResult result) {
            if (result.success) {
                qDebug(logging::sub()) << "Unwrapping stream controller";
                return result.payload.value;
            }

            throw app::CreateStreamFailed(result.payload.error);
        })
        .then(
            this,
            [this, videoSink](irt::StreamController *controller) {
                qDebug(logging::sub()) << "Ready to start";

                QPromise<irt::StreamController *> p;
                auto future = p.future();

                QObject::connect(this, &QObject::destroyed, [controller] {
                    qDebug(logging::sub()) << "Destroying stream";
                    irt::free_subscriber_stream(controller);
                });

                videoSink->window()->scheduleRenderJob(
                    QRunnable::create([p = std::move(p), controller]() mutable {
                        if (!irt::setup_subscriber_stream(controller)) {
                            throw app::SetupStreamFailed{};
                        }

                        p.addResult(controller);
                        p.finish();
                    }),
                    QQuickWindow::BeforeSynchronizingStage);

                return future;
            })
        .unwrap()
        .then(this,
              [this](irt::StreamController *controller) {
                  if (!irt::start_subscriber_stream(controller)) {
                      throw app::StartStreamFailed{};
                  }

                  status_.setValue(Playing);
              })
        .onFailed(this, [this](const std::exception &e) {
            qCritical(logging::sub()) << "Failed with exception:" << e.what();
            status_.setValue(Failed);
        });
}
} // namespace app