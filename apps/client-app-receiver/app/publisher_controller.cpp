#include "publisher_controller.h"

#include <QLoggingCategory>

#include "bridge/irt.h"
#include "constants.h"
#include "exceptions.h"
#include "utils.h"

namespace app {

namespace logging {
namespace {
Q_LOGGING_CATEGORY(pub, "app.pub")
}
} // namespace logging

void PublisherController::startPublishing(const QUrl &file) {
    status_.setValue(RequestingToken);

    app::utils::request_token(
        constants::serverUrl,
        irt::RoomOptions{.room_id = "room#1",
                         .identity = "sample-publisher",
                         .name = nullptr,
                         .role = irt::PeerRole::Publisher})
        .then(this,
              [this, file](irt::RequestResult result) {
                  utils::Defer _ =
                      std::bind_front(irt::free_request_result, result);

                  status_.setValue(StartingStream);

                  if (result.success) {
                      qDebug(logging::pub()) << "Creating stream object";
                      return irt::create_publisher_stream(
                          result.payload.value,
                          file.toString().toUtf8().constData());
                  }

                  throw app::RequestFailed(result.payload.error);
              })
        .then([](irt::CreatePublisherResult result) {
            if (result.success) {
                qDebug(logging::pub()) << "Unwrapping stream controller";
                return result.payload.value;
            }

            throw app::CreateStreamFailed(result.payload.error);
        })
        .then(this,
              [this](irt::StreamController *controller) {
                  qDebug(logging::pub()) << "Starting";

                  QObject::connect(this, &QObject::destroyed, [controller] {
                      qDebug(logging::pub()) << "Destroying stream";
                      irt::free_publisher_stream(controller);
                  });

                  if (!irt::start_publisher_stream(controller)) {
                      throw app::StartStreamFailed{};
                  }

                  status_.setValue(Playing);
              })
        .onFailed(this, [this](const std::exception &e) {
            qCritical(logging::pub()) << "Failed with exception:" << e.what();
            status_.setValue(Failed);
        });
}
} // namespace app