#ifndef IMMERSE_RT_APP_INSTANCE_H
#define IMMERSE_RT_APP_INSTANCE_H

#include <QObject>
#include <QtQml/qqmlregistration.h>

namespace app {

class AppInstance : public QObject {
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON

  public:
    enum ViewId { PublisherView, SubscriberView };

    Q_ENUM(ViewId)

    Q_INVOKABLE QObject *createController(ViewId viewId);

    Q_INVOKABLE void releaseController(QObject *object);
};

} // namespace app

#endif // IMMERSE_RT_APP_INSTANCE_H
