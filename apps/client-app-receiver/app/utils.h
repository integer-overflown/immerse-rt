#ifndef IMMERSE_RT_UTILS_H
#define IMMERSE_RT_UTILS_H

#include <functional>

#include <QFuture>
#include <QtConcurrent>
#include <QtGlobal>

#include "bridge/irt.h"

namespace app::utils {
template <typename Fn> class Defer {
  public:
    Defer(Fn &&fn) : fn_(std::forward<Fn>(fn)) {}

    Q_DISABLE_COPY_MOVE(Defer);

    ~Defer() noexcept(noexcept(fn_())) { fn_(); }

  private:
    Fn fn_;
};

inline QFuture<irt::RequestResult> request_token(const char *serverUrl,
                                                 irt::RoomOptions options) {
    return QtConcurrent::run([serverUrl, options] {
        return irt::request_token(serverUrl, options);
    });
}
} // namespace app::utils

#endif // IMMERSE_RT_UTILS_H
