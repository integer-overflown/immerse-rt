import QtQuick
import QtQuick.Layouts
import org.freedesktop.gstreamer.Qt6GLVideoItem 1.0

import com.github.immerse_rt

Item {
    required property var controller

    function statusString(status) {
        switch (status) {
            case SubscriberController.None:
                return "Standby"
            case SubscriberController.RequestingToken:
                return "Requesting token..."
            case SubscriberController.StartingStream:
                return "Starting stream..."
            case SubscriberController.Playing:
                return "Playing"
            case SubscriberController.Failed:
                return "Failed"
        }
    }

    Component.onCompleted: {
        controller.connectToStream(videoSink)
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillWidth: true
            Layout.fillHeight: true
            color: "black"

            GstGLQt6VideoItem {
                id: videoSink
                anchors.fill: parent
            }
        }

        Rectangle {
            Layout.fillWidth: true
            Layout.preferredHeight: 24

            color: controller.status === SubscriberController.Failed ? "#92241f" : "#00cc00"

            Text {
                anchors.fill: parent
                color: "white"

                font.weight: 600
                font.pointSize: 18
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter

                text: statusString(controller.status)
            }
        }
    }
}