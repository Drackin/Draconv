# Changelog

All notable changes to this project will be documented in this file.
The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/), and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2026-10-25
### Added
- Initial release of Draconv, a user-friendly video converter built with Tauri and FFmpeg.
- Added shadcn/nuxt to redesign UI for a more modern and intuitive user experience.
- Output file naming options added, allowing users to customize the naming convention for converted files.
- MacOS builds now seperated into two separate builds: one for Intel-based Macs and another for Apple Silicon-based Macs, ensuring optimal performance on both architectures.
- Added support for Apple Video Toolbox hardware acceleration on macOS, enabling faster video conversions on supported devices.
- Custom window title bar added for a more native look and feel on macOS, enhancing the overall user experience.
- Dynamic concurrency. The application now automatically starts or stops conversions when the user changes the max concurrency setting, providing a more responsive and efficient conversion process.
- Toast notifications.

### Changed
- Updated dependencies and libraries to their latest versions.
- Refactored codebase for better maintainability and performance.
- UI improvements and design changes for better user experience.

### Fixed
- Fixed bug when installing FFmpeg on macOS, the progress bar was not updating correctly.

## [1.0.0-beta] - 2025-02-08
### Added
- Initial release of Draconv, a user-friendly video converter built with Tauri and FFmpeg.
- Support for Windows, macOS, and Linux platforms.
- H.264, H.265, VP9, and AV1 video codecs with hardware acceleration support.
- Multiple file handling and batch processing capabilities.
- Max concurrency setting for controlling the number of simultaneous conversions.
- Donation links added to the app.
- Compact UI mode for a more streamlined experience.
- Added support for Apple Video Toolbox hardware acceleration on macOS.

### Changed
- UI improvements and design changes for better user experience.
- Refactored codebase for better maintainability and performance.
- Updated dependencies and libraries to their latest versions.

### Fixed
- Hardware acceleration detection and usage on different platforms.
- Various bug fixes and performance improvements.
- Various bugs related to file handling and conversion processes.