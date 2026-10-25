# Contributing to Ember

Thanks for taking a look. This is a small project — no process, no ceremony.

## Getting set up

See [Build from source](README.md#build-from-source) in the README. Short
version:

```bash
npm install
npm run dev
```

## Where things live

| I want to change… | Edit |
|---|---|
| Anything you can see | `src/index.html` |
| Window size, permissions | `src-tauri/tauri.conf.json` |
| Folder scanning, OAuth, opening links | `src-tauri/src/main.rs` |
| Release builds | `.github/workflows/release.yml` |

`src/index.html` is one file on purpose: no build step, no `node_modules`, no
bundler. Please keep it that way. If something needs a library, ask in an issue
first.

## Before you open a pull request

If you touched the Rust:

```bash
cd src-tauri
cargo fmt
cargo clippy --all-targets    # should be silent
cargo check
```

CI runs all three and will fail the PR otherwise.

If you touched the HTML, open it directly in a browser as well as in the app.
It has to keep working in both — the desktop features hide themselves when
`window.__TAURI__` is undefined, and that fallback should stay intact.

## Style

- Vanilla JavaScript, no framework.
- Every colour comes from a CSS custom property in `:root`. No hard-coded hex
  values further down the file.
- Comments explain *why*, not *what*. `// loop over tracks` is noise;
  `// bind before opening the browser, or a fast redirect hits a closed port`
  is worth having.

## Things that will be declined

- Anything that downloads audio from Spotify, YouTube, SoundCloud or similar.
  This is not negotiable — it is the fastest way to get a project taken down,
  and it takes money from artists.
- Analytics or telemetry of any kind.
- Adding a build pipeline to the frontend.

## Good first issues

- A new wallpaper preset (`PRESETS` array)
- A new accent colour (`SWATCHES` array)
- More audio extensions (`AUDIO` in `main.rs`)
- A light theme
