#ifndef IMMERSE_RT_SUBSCRIBER_CONTROLLER_H
#define IMMERSE_RT_SUBSCRIBER_CONTROLLER_H

#include <QObject>
#include <QQuickItem>

namespace app {

class SubscriberController : public QObject {
    Q_OBJECT
    Q_PROPERTY(
        Status status READ status BINDABLE bindableStatus NOTIFY statusChanged)
    QML_ELEMENT
    QML_UNCREATABLE("Managed by C++ layer")

  public:
    enum Status { None, RequestingToken, StartingStream, Playing, Failed };

    Q_ENUM(Status)

    Q_INVOKABLE void connectToStream(QQuickItem *videoSink);

    [[nodiscard]] QBindable<Status> bindableStatus() const { return &status_; }

    [[nodiscard]] Status status() const { return status_.value(); }

  signals:
    void statusChanged();

  private:
    Q_OBJECT_BINDABLE_PROPERTY(SubscriberController, Status, status_,
                               &SubscriberController::statusChanged);
};

} // namespace app

#endif // IMMERSE_RT_SUBSCRIBER_CONTROLLER_H
