# Changelog

All notable changes to Ember are recorded here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project uses [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added
- Gallery: browse Audius by mood and genre without adding anything first
- Settings: startup view, intro toggle, autoplay, volume, data reset with confirmation
- Spotify Client ID can be pasted in Settings (no rebuild)
- YouTube playlist import through the official API (needs a key)
- Discord Rich Presence (needs an application ID)
- Redesigned boot animation with equalizer and sparks

### Fixed
- Hidden elements (badges, Now Playing panel, empty images) could still show
- Offline gallery pages had no way back
- Phone-width layout grid rows

### Ideas
- Light theme
- YouTube playlist import
- Gapless playback
- Last.fm scrobbling

## [0.1.0] — 2026-10-24

First release.

### Added
- Streaming from the Audius network: trending, underground, genre filters, search
- Local music library — folder scanning with ID3 tags and embedded cover art
- Playlists, liked tracks, play queue
- Import from an Audius link, a pasted song list, or a connected Spotify account
- Customisation: wallpaper upload, blur, darkness, accent colour
- "Reduce effects" switch for low-end hardware
- Keyboard shortcuts and OS media-key support
- Native builds for Windows, macOS and Linux

[Unreleased]: https://github.com/YOUR-USERNAME/ember/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/YOUR-USERNAME/ember/releases/tag/v0.1.0
