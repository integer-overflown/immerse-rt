import QtQuick
import org.freedesktop.gstreamer.Qt6GLVideoItem 1.0

Item {
    Rectangle {
        color: "black"
        anchors.fill: parent

        GstGLQt6VideoItem {
            anchors.fill: parent
            objectName: "videoItem"
        }
    }
}