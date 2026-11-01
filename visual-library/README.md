# Moonbyte Visual Library

The first Next.js + React + TypeScript presentation layer for Moonbyte. It is deliberately a **catalogue and command handoff**, not a fake browser game client.

## Requirements

- Node.js 20.9 or later
- npm

## Run locally

```sh
cd visual-library
npm install
npm run dev
```

Open the local URL printed by Next.js. Use `npm run typecheck` and `npm run build` before publishing.

## Current behavior

- Responsive catalogue of the four games implemented in the Rust bot runtime.
- Search and category filters.
- Accessible controls disclosure and reduced-motion support.
- Copyable Discord play commands.
- Original CSS pixel-inspired artwork, with no external game sprites.
- Browser-first fallback; no Discord Activity handshake or backend API is claimed.

## Architecture boundary

This frontend is presentation-only. It does not decide legal moves, player identity, guild membership, session phase, outcomes, rewards, balances, trade rules, or persistence. Rust remains authoritative. The catalogue is intentionally limited to implemented games so planned features are not presented as playable.

The next integration step is to add the official Discord Embedded App SDK behind a narrowly scoped Activity adapter and define an authenticated backend contract. Do not accept user identity or game outcomes from browser-supplied values. An Activity handshake is not a substitute for backend authorization.

## Design direction

Original full-colour pixel-world motifs, warm paper surfaces, deep forest ink, and a lime accent. Layouts reflow for narrow mobile screens, focus indicators remain visible, and reduced-motion preferences are respected. The current hero landscape is CSS artwork; it is not presented as a shipped sprite asset.
