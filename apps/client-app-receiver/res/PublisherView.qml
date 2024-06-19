import QtCore
import QtQuick
import QtQuick.Dialogs
import QtQuick.Controls
import QtQuick.Layouts

import com.github.immerse_rt

Item {
    id: screen

    state: "selecting-source"

    function navigate(stackView, stateId) {
        const getComponent = state => {
            switch (state) {
                case "selecting-source":
                    return [selectSourceView, 0]
                case "verifying":
                    return [verifyView, 1]
                case "streaming":
                    return [streamingView, 2]
                default: {
                    console.error(`Cannot match component for state ${state}`)
                    return undefined
                }
            }
        }

        const currentComponentDesc = getComponent(stateId)

        if (!currentComponentDesc) {
            return
        }

        const [currentComponent, currentOrder] = currentComponentDesc
        const previousOrder = stackView.currentItem ? stackView.currentItem.order : -1

        console.log(`Transitioning: ${previousOrder} -> ${currentOrder}`)

        if (currentOrder < previousOrder) {
            let steps = previousOrder - currentOrder

            for (let i = 0; i < steps; i++) {
                console.log("popping")
                stackView.pop()
            }
        } else {
            stackView.push(currentComponent.createObject(stackView))
        }
    }

    states: [
        State {
            name: "selecting-source"

            StateChangeScript {
                script: {
                    screen.navigate(stackView, "selecting-source");
                }
            }
        },
        State {
            name: "verifying"

            StateChangeScript {
                script: {
                    screen.navigate(stackView, "verifying");
                }
            }
        },
        State {
            name: "streaming"

            StateChangeScript {
                script: {
                    screen.navigate(stackView, "streaming");
                }
            }
        }
    ]

    FileDialog {
        id: fileDialog

        currentFolder: StandardPaths.standardLocations(StandardPaths.HomeLocation)[0]
        nameFilters: ["Video files (*.mov *.mp4)"]
        title: qsTr("Please select a video file")

        onAccepted: {
            console.log("Chosen file: " + fileDialog.selectedFile);
            screen.state = "verifying";
        }
        onRejected: {
            console.log("Canceled");
        }
    }
    StackView {
        id: stackView

        anchors.fill: parent

        pushEnter: Transition {
            YAnimator {
                duration: 400
                easing.type: Easing.OutCubic
                from: stackView.height
                to: 0
            }
        }

        pushExit: Transition {
            YAnimator {
                duration: 400
                easing.type: Easing.OutCubic
                from: 0
                to: -stackView.height
            }
        }

        popEnter: Transition {
            YAnimator {
                duration: 400
                easing.type: Easing.OutCubic
                from: -stackView.height
                to: 0
            }
        }

        popExit: Transition {
            YAnimator {
                duration: 400
                easing.type: Easing.OutCubic
                from: 0
                to: -stackView.height
            }
        }
    }

    Component {
        id: selectSourceView

        Item {
            readonly property int order: 0

            ColumnLayout {
                anchors.centerIn: parent
                spacing: 8

                Column {
                    Layout.fillWidth: true

                    Text {
                        color: "white"
                        font.pointSize: 36
                        text: qsTr("Select a video file")
                    }
                    Text {
                        color: "grey"
                        font.pointSize: 18
                        text: qsTr("...and we'll do the rest 😉")
                    }
                }
                Button {
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    text: "Select"

                    onClicked: {
                        fileDialog.open();
                    }

                    HoverHandler {
                        id: mouse

                        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                        cursorShape: Qt.PointingHandCursor
                    }
                }
            }
        }
    }
    Component {
        id: verifyView

        Item {
            readonly property int order: 1

            ColumnLayout {
                anchors.centerIn: parent
                spacing: 8

                Text {
                    color: "white"
                    font.pointSize: 24
                    text: qsTr("Let's see if we got it right 🧐")
                }

                Text {
                    color: "white"
                    font.pointSize: 18
                    text: "Selected file: " + `<b>${AppQmlUtils.toLocalFilePath(fileDialog.selectedFile)}</b>`
                }

                RowLayout {
                    Layout.fillWidth: true
                    spacing: 4

                    Button {
                        Layout.alignment: Qt.AlignVCenter
                        text: "Back"

                        onClicked: {
                            screen.state = "selecting-source"
                        }
                    }

                    Button {
                        Layout.alignment: Qt.AlignVCenter
                        Layout.fillWidth: true

                        text: "Start streaming"

                        onClicked: {
                            screen.state = "streaming"
                        }

                        HoverHandler {
                            acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
                            cursorShape: Qt.PointingHandCursor
                        }
                    }
                }
            }
        }
    }
    Component {
        id: streamingView

        Item {
            readonly property int order: 2

            property var controller: AppInstance.createController(AppInstance.PublisherView)

            Component.onDestruction: {
                console.log("Destroying")
                AppInstance.releaseController(controller)
            }

            ColumnLayout {
                anchors.centerIn: parent
                spacing: 16

                Text {
                    text: "Streaming..."
                    color: "white"
                    font.pointSize: 36
                }

                Button {
                    Layout.fillWidth: true

                    text: "Stop streaming"

                    onClicked: {
                        screen.state = "selecting-source"
                    }
                }
            }
        }
    }
}