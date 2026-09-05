---
layout: home
title: WRY Unity
titleTemplate: Native WebViews for Unity
description: Embed browser-based login, CAPTCHA, account, and announcement screens in Unity.
hero:
  name: WRY Unity
  text: Native WebViews for Unity games and applications
  tagline: Build login screens, CAPTCHA flows, account linking, and announcements without rebuilding browser behavior in Unity.
  image:
    src: /logo.svg
    alt: WRY Unity logo
  actions:
    - theme: brand
      text: Start integrating
      link: /getting-started
    - theme: alt
      text: Read the architecture
      link: /ARCHITECTURE
features:
  - title: Native overlays
    details: WebView2, WebKitGTK, Android WebView, and WKWebView sit above the Unity player as real native views.
  - title: C# and JavaScript IPC
    details: Send messages in either direction and receive page-load, navigation, and script-evaluation callbacks.
  - title: Unity 2021 and Unity 6
    details: A small C# package works with Mono and IL2CPP, while the native layer stays independent of Unity's render pipeline.
---

## What this project is

WRY Unity is an embeddable fork of Tauri WRY. It is designed for applications
where a Unity player needs a browser surface for a small part of the product:

- account login and account linking
- CAPTCHA and identity verification
- announcements and news
- store, support, or community pages
- web-based payment and consent flows

The WebView is a native overlay. It is not a screenshot of a browser and it is
not a replacement for Unity UI.

## Where to go next

If you are integrating the package, start with [Getting Started](./getting-started).
If you are deciding whether the approach fits your game, read [Platform
Overview](./platforms). If you are working on the native side, read [Native
API](./native-api) and [Architecture](./ARCHITECTURE).

## Project status

The repository provides the Rust bridge, C ABI, Unity package source, Android
adapter source, and iOS build hook. Native libraries must be built for the
architectures you ship; prebuilt binaries are intentionally not committed.

Desktop Linux verification is available on a Linux host with GTK and WebKitGTK.
Android, iOS, and Windows builds require their platform SDKs.
