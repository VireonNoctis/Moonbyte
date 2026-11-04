# Moonbyte Visual Library — Design System

> Visual direction and implementation contract for the Next.js + React field guide.
> Status: first design-system implementation; live game/session integration is not yet shipped.

## Product role

The Visual Library is the visual front door to Moonbyte. It should make the existing Discord experience understandable and inviting, show the real playable catalogue, and help a player move from discovery to a Discord command without pretending that a browser-only page is a live game client.

The design follows the original product promise: a colourful pocket arcade connected to a persistent world. The interface is a field guide to that world, not a generic SaaS dashboard and not a feed.

## Visual language

- **Colour:** warm paper, deep forest ink, moss, lime, and distinct coral, sun, lilac and blue accents. Use colour to distinguish places and game families; never rely on colour alone for meaning.
- **Artwork:** original full-colour pixel-inspired scenes built from authored CSS geometry and small interface motifs. Do not copy protected console branding, game sprites or unlicensed art. A CSS illustration is a design asset, not a claim that the runtime contains a shipped sprite sheet.
- **Typography:** system sans-serif for readable body copy, a compact display face for headings, and monospaced labels for route IDs, section numbers and technical metadata. No external font CDN is required.
- **Surfaces:** warm paper cards, printed field notes, map panels and slightly offset shadows. Keep depth restrained and purposeful.
- **Controls:** tactile buttons use a firm lower edge, a small hover lift and a clear pressed state. Interactions should feel deliberate without making every element bounce.
- **Motion:** brief, low-amplitude transitions communicate selection, copying, disclosure and navigation. Honor reduced-motion preferences and never hide information behind animation.

The visual studies draw from the physical-button and receipt-like interaction experiments in Dqnamo Kitchen, and the purposeful semantic motion demonstrated by pqoqubbw/icons. The implementation is original CSS and React; it does not redistribute their component source or assets.

## Page structure

1. **Field Guide hero:** product promise, direct catalogue and Atlas entry points, and an original CSS landscape illustration.
2. **Current build facts:** four playable games, five authored Atlas locations and eleven registered Tavern destinations.
3. **Common Tables:** searchable, filterable cards for the four implemented games. Each card shows player count, typical session range, controls and the real prefix command.
4. **World Index:** a selectable map of the five authored Atlas locations. Route lines match the Rust catalogue. Selecting a place reveals its region, description, Chronicle title and direct neighbours.
5. **Tavern Wayfinder:** the eleven destinations currently backed by registered Rust commands. The command list is sourced from src/tavern.rs and should be kept in sync with its tests.
6. **Wayfarer Thread:** the fixed five-objective catalogue, cumulative journey principles and the .quests entry point.
7. **Product principles:** explain play without pressure, connected systems and honest feature status.

## Truthful data contract

The public field guide is not authenticated and has no live player-view API. It must not invent or cache personal state.

- Game names, commands, player counts, estimated durations and controls come from the reviewed current catalogue.
- The Atlas shows the authored world graph only. It does not claim to show the visitor's current location or discoveries.
- Tavern destinations are listed only if a registered command backs them.
- The Wayfarer section shows the five possible objective definitions. Three are selected per player/day; the site does not guess which three were assigned.
- Wallet balances, player profiles, activity history, rivalry records, current quests and Chronicle unlock status remain private inside Discord.
- A button may copy a command. It must not imply that it starts a game, joins a session or changes saved progress.

## Component boundaries

- Server components own static editorial structure and reviewed public catalogue data.
- Small client components own search/filter state, map selection and clipboard interactions.
- Rust remains authoritative for player identity, game rules, sessions, permissions, outcomes, progression and persistence.
- The official Discord Embedded App SDK and authenticated Rust view/action API are future work. Do not accept user identity or game outcomes from browser-supplied fields.

## Accessibility and responsive behavior

- Support narrow mobile layouts without horizontal page overflow.
- Keep keyboard focus visible and use native buttons, links, inputs, details/summary and labelled navigation.
- Announce changing game-result counts and selected Atlas details appropriately.
- Keep contrast and text labels usable when colour is unavailable.
- Ensure every map relationship has a written direct-route list.
- Respect prefers-reduced-motion and make the page complete without animation.
- Avoid hover-only information and make touch targets comfortably usable.

## Engineering checks

From visual-library:

- npm install --no-audit --no-fund --no-package-lock
- npm run typecheck
- npm run build

The GitHub Actions workflow runs the visual-library typecheck and production build separately from the Rust/PostgreSQL job. Passing the frontend build does not imply that the Discord Activity handshake or game API exists.
