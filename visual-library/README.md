# Moonbyte Visual Library

The Moonbyte Field Guide is the public-facing visual entry point for the Discord game world. It is built with Next.js, React and TypeScript, with original CSS artwork and a responsive, tactile interface.

## Requirements

- Node.js 22 or later
- npm

## Run locally

    cd visual-library
    npm install
    npm run dev

Use npm run typecheck and npm run build before publishing.

## Current experience

- A redesigned field-guide landing page with original pixel-inspired CSS artwork.
- Searchable and filterable cards for the four games currently implemented in the Rust runtime.
- Four statically generated game guide routes under /games/[slug], sourced from the same catalogue as the landing page.
- Reusable React artwork, section-label and guide-fact components shared by the game catalogue and detail pages.
- Per-game command handoff, session type, player count, estimated duration, controls and explicit starter-build notes where relevant.
- An interactive Atlas map showing the five authored locations and their direct route relationships.
- The eleven Tavern destinations currently backed by registered commands.
- The five fixed Wayfarer daily-objective definitions, without fabricating a player's daily assignment or progress.
- Copyable Discord commands, keyboard-accessible disclosures, visible focus states and reduced-motion support.
- Mobile layouts designed for touch screens as well as desktop browsing.

## Architecture boundary

This frontend is presentation-only. It does not decide legal moves, player identity, guild membership, session phase, outcomes, rewards, balances, trade rules or persistence. Rust remains authoritative.

The Atlas section shows the static authored map, not a visitor's private location. The Wayfarer section shows the objective catalogue, not a player's assignment. Profile, wallet, stats, current quest state, Chronicle unlocks and rivalry history remain private in Discord until an authenticated Rust view API exists.

The game detail pages are generated from the four implemented catalogue entries. They explain the actual command and controls, but do not start sessions, accept player actions or emulate game rules. Planned games do not have detail routes.

The official Discord Embedded App SDK handshake and authenticated game API are not implemented by this visual-library slice. Do not accept user identity or game outcomes from browser-supplied values. An Activity handshake is not a substitute for backend authorization.

## Design references and original work

The visual direction combines original full-colour pixel-world motifs, warm paper surfaces, forest ink and lime accents. Tactile depth and purposeful motion are informed by interaction studies such as Dqnamo Kitchen and pqoqubbw/icons. The code and artwork in this repository are original implementations; third-party component source and protected game art are not copied into the project.

See docs/VISUAL_LIBRARY.md for the design and accessibility contract.
