"use client";

import { useMemo, useState } from "react";
import Link from "next/link";
import { CopyCommandButton } from "@/components/copy-command";
import { StatusIndicator } from "@/components/ui/status-indicator";
import { GameArtwork } from "@/components/game-artwork";
import { categories, games, type CategoryFilter, type GameDefinition } from "@/lib/catalog";

function GameCard({ game, index }: { game: GameDefinition; index: number }) {
  return (
    <article className={"game-card game-card--" + game.palette}>
      <div className="game-card__art-wrap">
        <div className="game-card__topline">
          <span className="game-card__category">{game.category}</span>
          <StatusIndicator className="playable-label" label="PLAYABLE" />
        </div>
        <GameArtwork game={game} />
        <span className="game-card__index">MB / 0{index + 1}</span>
      </div>

      <div className="game-card__body">
        <div className="game-card__heading">
          <div>
            <span className="game-card__edition">THE CURRENT LINE-UP</span>
            <h3>{game.name}</h3>
          </div>
          <span className="game-card__serial" aria-hidden="true">0{index + 1}</span>
        </div>
        <p className="game-card__description">{game.description}</p>

        <div className="game-card__facts">
          <span><i aria-hidden="true">♟</i>{game.players}</span>
          <span><i aria-hidden="true">◷</i>{game.duration}</span>
        </div>

        <details className="game-card__guide">
          <summary>
            <span>Controls & quick start</span>
            <span className="game-card__guide-symbol" aria-hidden="true">＋</span>
          </summary>
          <div className="game-card__guide-content">
            <p>{game.controls}</p>
            <code>{game.command}</code>
          </div>
        </details>

        <div className="game-card__actions">
          <CopyCommandButton value={game.command} className="copy-button copy-button--primary">
            Copy play command
          </CopyCommandButton>
          <Link className="game-guide-link" href={"/games/" + game.id}>
            Full game guide <span aria-hidden="true">↗</span>
          </Link>
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
        game.name,
        game.description,
        game.category,
        game.players,
      ].some((value) => value.toLocaleLowerCase().includes(normalizedQuery));

      return matchesCategory && matchesQuery;
    });
  }, [category, query]);

  function resetFilters() {
    setQuery("");
    setCategory("All games");
  }

  return (
    <>
      <div className="library-toolbar">
        <label className="search-box">
          <span className="search-box__icon" aria-hidden="true">⌕</span>
          <span className="sr-only">Search playable games</span>
          <input
            onChange={(event) => setQuery(event.target.value)}
            placeholder="Search the current line-up…"
            type="search"
            value={query}
          />
          <kbd>/</kbd>
        </label>

        <div className="filter-list" aria-label="Filter by category" role="group">
          {categories.map((item) => (
            <button
              aria-pressed={category === item}
              className={"filter-chip" + (category === item ? " filter-chip--active" : "")}
              key={item}
              onClick={() => setCategory(item)}
              type="button"
            >
              {item}
            </button>
          ))}
        </div>
      </div>

      <div className="library-results" aria-live="polite">
        <span><strong>{filteredGames.length.toString().padStart(2, "0")}</strong> {filteredGames.length === 1 ? "game" : "games"} found</span>
        <span>SHORT SESSIONS · EASY EXITS · REAL COMMANDS</span>
      </div>

      {filteredGames.length > 0 ? (
        <div className="game-grid">
          {filteredGames.map((game) => (
            <GameCard game={game} index={games.findIndex((item) => item.id === game.id)} key={game.id} />
          ))}
        </div>
      ) : (
        <div className="empty-state">
          <span className="empty-state__symbol" aria-hidden="true">⌁</span>
          <h3>Nothing on this shelf.</h3>
          <p>Try another title or clear the current filters.</p>
          <button className="button button--secondary" onClick={resetFilters} type="button">
            Reset filters <span aria-hidden="true">↺</span>
          </button>
        </div>
      )}

      <p className="library-footnote">
        Commands use the default period prefix. A server may have its own prefix configured by a moderator.
      </p>
    </>
  );
}
