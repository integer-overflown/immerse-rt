import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

import com.github.immerse_rt

ApplicationWindow {
    height: 480
    title: qsTr("immerse-rt")
    visible: true
    width: 640

    StackView {
        id: stackView

        anchors.fill: parent
        initialItem: mainView
    }
    Component {
        id: mainView

        Item {
            id: root

            function navigate(componentName, properties) {
                const component = Qt.createComponent(componentName);
                if (component.status !== Component.Ready) {
                    console.error(`Failed to load view: ${component.errorString()}`);
                    return;
                }
                stackView.push(component.createObject(stackView, properties));
            }

            ColumnLayout {
                anchors.centerIn: parent
                width: parent.width / 4
                spacing: 4

                Button {
                    Layout.fillWidth: true
                    text: qsTr("Host a room")
                    onClicked: {
                        root.navigate("PublisherView.qml")
                    }
                }

                Button {
                    Layout.fillWidth: true
                    text: qsTr("Join a room")

                    onClicked: {
                        root.navigate("SubscriberView.qml", {
                            controller: AppInstance.createController(AppInstance.SubscriberView)
                        });
                    }
                }
            }
        }
    }
}
