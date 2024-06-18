#include <QtQml/qqmlregistration.h>

#include <QObject>
#include <QUrl>

namespace app {

class AppQmlUtils : public QObject {
    Q_OBJECT
    QML_ELEMENT
    QML_SINGLETON

  public slots:
    QString toLocalFilePath(const QUrl &url) {
        Q_UNUSED(this)
        return url.toLocalFile();
    }
};

} // namespace app