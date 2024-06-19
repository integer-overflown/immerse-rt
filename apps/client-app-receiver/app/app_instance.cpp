#include "app_instance.h"

#include <QQmlApplicationEngine>

#include "publisher_controller.h"
#include "subscriber_controller.h"

namespace app {

namespace {
QObject *matchController(AppInstance::ViewId viewId) {
    using enum AppInstance::ViewId;

    switch (viewId) {
    case PublisherView:
        return new PublisherController;
    case SubscriberView:
        return new SubscriberController;
    default:
        qFatal() << "Unhandled view id" << viewId;
    }
}
} // namespace

QObject *AppInstance::createController(AppInstance::ViewId viewId) {
    auto *view = matchController(viewId);
    QQmlApplicationEngine::setObjectOwnership(
        view, QQmlApplicationEngine::CppOwnership);

    return view;
}

void AppInstance::releaseController(QObject *object) { object->deleteLater(); }

} // namespace app