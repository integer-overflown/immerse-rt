import QtQuick 2.15
import QtQuick.Controls 2.15

ApplicationWindow {
    height: 480
    title: "Hello, Qt QML!"
    visible: true
    width: 640

    Button {
        anchors.centerIn: parent
        text: "Click Me"
    }
}
