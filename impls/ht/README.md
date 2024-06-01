# Head tracking module

This folder contains different head-tracking implementations, based on a platform and API.

Current implementations include:

| Folder                   | Description                                                                                                          |
|--------------------------|----------------------------------------------------------------------------------------------------------------------|
| api                      | A facade implementation, providing entry point for getting platform-specific head-tracking API implementations.      |
| core-motion (macOS only) | Swift-based implementation built on top of CoreMotion API. <br/>Requires user to have eligible device, e.g. AirPods. |
