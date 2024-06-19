#ifndef IMMERSE_RT_PUBLISHER_CONTROLLER_H
#define IMMERSE_RT_PUBLISHER_CONTROLLER_H

#include <QObject>

namespace app {

class PublisherController : public QObject {
    Q_OBJECT

  public:
    void startPublishing(const QUrl &file);
};

} // namespace app

#endif // IMMERSE_RT_PUBLISHER_CONTROLLER_H
