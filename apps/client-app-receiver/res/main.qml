import QtQuick 2.15
import QtQuick.Controls 2.15

import org.freedesktop.gstreamer.Qt6GLVideoItem 1.0

ApplicationWindow {
    height: 480
    title: qsTr("immerse-rt")
    visible: true
    width: 640

    GstGLQt6VideoItem {
        anchors.fill: parent
        objectName: "videoItem"
    }
}
