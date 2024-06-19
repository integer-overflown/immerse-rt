#ifndef IMMERSE_RT_UTILS_H
#define IMMERSE_RT_UTILS_H

#include <functional>

#include <QtGlobal>

namespace app::utils {
template <typename Fn> class Defer {
  public:
    Defer(Fn &&fn) : fn_(std::forward<Fn>(fn)) {}

    Q_DISABLE_COPY_MOVE(Defer);

    ~Defer() noexcept(noexcept(fn_())) { fn_(); }

  private:
    Fn fn_;
};
} // namespace app::utils

#endif // IMMERSE_RT_UTILS_H
