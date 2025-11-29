# Mafia Engine MS

This monorepo contains toolings and components for MafiaScum. This includes a browser extension, a website and a forum bot to help predominately in hosting games of Mafia.

## Repository Structure

Standalone "binaries" are in the `app` folder whereas shared logic is in `/shared`. The shared folder is almost entirely for Rust that can be compiled to both native and WebAssembly.

## Apps
- **Server:** The core backend that handles auth, a web dashboard, and exposes APIs for the extension and workers.
- **Extension:** Browser extension that users install to have Mafia Engine interact with the forum directly.
- **Browser Worker:** Background process that runs a headless Chromium instance to interact with the forum. Can be run inside the server as well as in its own instance (connected via API) if need arises where the server is overloaded.

### Shared
- **Votecounter:** all vote-counting logic to keep logic consistent between the server and clients.
- **Crawler:** gather data from the forum in a lightweight and client-focused way so it can be performed "offline" when the server is unavailable.
