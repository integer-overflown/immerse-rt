#ifndef IMMERSE_RT_EXCEPTIONS_H
#define IMMERSE_RT_EXCEPTIONS_H

#include <format>
#include <stdexcept>

#include "bridge/irt.h"

namespace app {
class RequestFailed : public std::runtime_error {
  public:
    explicit RequestFailed(irt::RequestErrorCode e)
        : std::runtime_error(
              std::format("Request failed with {}", int(e)).c_str()) {}
};

class FailedToReadHRIR : public std::runtime_error {
  public:
    FailedToReadHRIR() : std::runtime_error("Cannot read HRIR resource") {}
};

class CreateStreamFailed : public std::runtime_error {
  public:
    explicit CreateStreamFailed(irt::CreateSubscriberErrorCode e)
        : CreateStreamFailed(int(e)) {}

    explicit CreateStreamFailed(irt::CreatePublisherErrorCode e)
        : CreateStreamFailed(int(e)) {}

  private:
    explicit CreateStreamFailed(int code)
        : std::runtime_error(
              std::format("Cannot create stream: {}", code).c_str()) {}
};

class SetupStreamFailed : public std::runtime_error {
  public:
    SetupStreamFailed() : std::runtime_error("Failed to setup stream") {}
};

class StartStreamFailed : public std::runtime_error {
  public:
    StartStreamFailed() : std::runtime_error("Failed to start stream") {}
};

} // namespace app

#endif // IMMERSE_RT_EXCEPTIONS_H
