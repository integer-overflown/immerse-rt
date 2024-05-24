# Immerse-rt

Home to immerse-rt project, the real-time spatial audio engine
reliant on HRTF algorithms and listener position awareness 🎧

The primary purpose of this project is bringing immersive feel to live streams,
but the list of its potential applications is surely not limited to a single use-case.

## Project layout

| Folder  | Description                                                                                                                                                                                      |
|---------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| apps/   | End-user applications that combine libraries and implementations for implementing particular use-cases                                                                                           |
| cmake/  | CMake module directory; contains utilities and platform-specific build logic                                                                                                                     |
| config/ | Data files, mostly platform-specific, bundled with the application                                                                                                                               |
| impls/  | Platform- or API-specific implementations for the concepts described in library crates                                                                                                           |
| libs/   | Crates that provide library interface and generic logic and calculations. The actual implementations are located in impls/ folder and are plugged-in based on the platform, build settings, etc. |

## Building the project

The primary concepts, calculations and logic are implemented in Rust.
However, different implementations require platform-specific code, which often leverages the corresponding
languages.

Such implementations are built with CMake, which is extensible and supports many platforms and languages.

TODO: describe actual build instructions
