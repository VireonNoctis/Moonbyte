"use client";

import { useMemo, useState } from "react";
import { categories, games, type CategoryFilter, type GameDefinition } from "@/lib/catalog";

function GameCard({ game }: { game: GameDefinition }) {
  const [showControls, setShowControls] = useState(false);
  const [copied, setCopied] = useState(false);

  async function copyCommand() {
    try {
      await navigator.clipboard.writeText(game.command);
      setCopied(true);
      window.setTimeout(() => setCopied(false), 1600);
    } catch {
      setCopied(false);
      setShowControls(true);
    }
  }

  return (
    <article className={`game-card game-card--${game.palette}`}>
      <div className="game-card__topline">
        <span className="game-card__category">{game.category}</span>
        <span className="status-dot"><span aria-hidden="true" />Playable in Discord</span>
      </div>
      <div className="game-card__art" aria-hidden="true">
        <span className="game-card__glyph">{game.glyph}</span>
        <span className="game-card__art-grid" />
        <span className="game-card__art-index">{game.id.slice(0, 2).toUpperCase()}</span>
      </div>
      <div className="game-card__body">
        <h3>{game.name}</h3>
        <p>{game.description}</p>
        <div className="game-card__meta">
          <span><span aria-hidden="true">♟</span>{game.players}</span>
          <span><span aria-hidden="true">◷</span>{game.duration}</span>
        </div>
        {showControls && (
          <div className="game-card__controls" id={`controls-${game.id}`}>
            <p>{game.controls}</p>
            <code>{game.command}</code>
          </div>
        )}
        <div className="game-card__actions">
          <button className="button button--primary" onClick={copyCommand} type="button">
            {copied ? "Command copied ✓" : "Copy play command"}
          </button>
          <button
            aria-controls={`controls-${game.id}`}
            aria-expanded={showControls}
            className="button button--quiet"
            onClick={() => setShowControls((visible) => !visible)}
            type="button"
          >
            {showControls ? "Hide rules" : "How to play"}
          </button>
        </div>
      </div>
    </article>
  );
}

export function GameLibrary() {
  const [query, setQuery] = useState("");
  const [category, setCategory] = useState<CategoryFilter>("All games");
  const filteredGames = useMemo(() => {
    const normalizedQuery = query.trim().toLocaleLowerCase();
    return games.filter((game) => {
      const matchesCategory = category === "All games" || game.category === category;
      const matchesQuery = !normalizedQuery || [
        game.name, game.description, game.category, game.players,
      ].some((value) => value.toLocaleLowerCase().includes(normalizedQuery));
      return matchesCategory && matchesQuery;
    });
  }, [category, query]);

  return (
    <>
      <section className="library-tools" aria-label="Filter games">
        <label className="search-box">
          <span aria-hidden="true" className="search-box__icon">⌕</span>
          <span className="sr-only">Search games</span>
          <input
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Find your next game…"
            type="search"
            value={query}
          />
        </label>
        <div className="filter-list" aria-label="Game category" role="group">
          {categories.map((item) => (
            <button
              aria-pressed={category === item}
              className={`filter-chip ${category === item ? "filter-chip--active" : ""}`}
              key={item}
              onClick={() => setCategory(item)}
              type="button"
            >
              {item}
            </button>
          ))}
        </div>
      </section>
      <div className="library-results" aria-live="polite">
        <span>{filteredGames.length} {filteredGames.length === 1 ? "game" : "games"}</span>
        <span>Built for short sessions, shared servers, and easy exits.</span>
      </div>
      {filteredGames.length > 0 ? (
        <div className="game-grid">
          {filteredGames.map((game) => <GameCard game={game} key={game.id} />)}
        </div>
      ) : (
        <div className="empty-state">
          <span aria-hidden="true">⌁</span>
          <h3>No games found</h3>
          <p>Try another title or switch the category filter.</p>
          <button className="button button--quiet" onClick={() => { setQuery(""); setCategory("All games"); }} type="button">
            Reset filters
          </button>
        </div>
      )}
    </>
  );
}
