#ifndef IMMERSE_RT_PUBLISHER_CONTROLLER_H
#define IMMERSE_RT_PUBLISHER_CONTROLLER_H

#include <QBindable>
#include <QObject>
#include <QUrl>
#include <QtQml/qqmlregistration.h>

namespace app {

class PublisherController : public QObject {
    Q_OBJECT
    Q_PROPERTY(
        Status status READ status BINDABLE bindableStatus NOTIFY statusChanged)
    QML_ELEMENT
    QML_UNCREATABLE("Managed by C++ layer")

  public:
    enum Status {
        None [[maybe_unused]],
        RequestingToken,
        StartingStream,
        Playing,
        Failed
    };

    Q_ENUM(Status)

    Q_INVOKABLE void startPublishing(const QUrl &file);

    [[nodiscard]] QBindable<Status> bindableStatus() const { return &status_; }

    [[nodiscard]] Status status() const { return status_.value(); }

  signals:
    void statusChanged();

  private:
    Q_OBJECT_BINDABLE_PROPERTY(PublisherController, Status, status_,
                               &PublisherController::statusChanged);
};

} // namespace app

#endif // IMMERSE_RT_PUBLISHER_CONTROLLER_H
