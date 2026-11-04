import type { GameDefinition } from "@/lib/catalog";

type GameArtworkProps = {
  game: GameDefinition;
  featured?: boolean;
};

export function GameArtwork({ game, featured = false }: GameArtworkProps) {
  const modifier = featured ? " game-art--featured" : "";

  if (game.id === "tic-tac-toe") {
    const marks = ["×", "", "○", "", "×", "", "○", "", "×"];
    return (
      <div className={"game-art game-art--tic" + modifier} aria-hidden="true">
        <span className="game-art__orbit" />
        <div className="tic-board">
          {marks.map((mark, index) => (
            <span className={mark === "×" ? "tic-board__cross" : mark === "○" ? "tic-board__circle" : ""} key={index}>
              {mark}
            </span>
          ))}
        </div>
        <span className="game-art__art-label">THREE IN A ROW</span>
      </div>
    );
  }

  if (game.id === "connect-four") {
    const discs = new Map<number, string>([
      [8, "disc--coral"], [9, "disc--sun"], [13, "disc--coral"],
      [14, "disc--sun"], [15, "disc--coral"], [19, "disc--sun"],
    ]);
    return (
      <div className={"game-art game-art--connect" + modifier} aria-hidden="true">
        <div className="connect-board">
          {Array.from({ length: 42 }, (_, index) => (
            <span key={index}><i className={discs.has(index) ? discs.get(index) : ""} /></span>
          ))}
        </div>
        <span className="game-art__art-label">DROP · CONNECT · WIN</span>
      </div>
    );
  }

  if (game.id === "memory-match") {
    const symbols = ["✦", "◈", "✦", "⌂", "◌", "◈", "⌂", "◌"];
    return (
      <div className={"game-art game-art--memory" + modifier} aria-hidden="true">
        <div className="memory-board">
          {symbols.map((symbol, index) => (
            <span className={index === 0 || index === 1 || index === 2 ? "memory-tile memory-tile--shown" : "memory-tile"} key={index}>
              {index === 0 || index === 1 || index === 2 ? symbol : "?"}
            </span>
          ))}
        </div>
        <span className="game-art__art-label">REMEMBER THE PATTERN</span>
      </div>
    );
  }

  return (
    <div className={"game-art game-art--word" + modifier} aria-hidden="true">
      <div className="word-art">
        <span className="word-art__letter word-art__letter--large">A</span>
        <span className="word-art__letter">b</span>
        <span className="word-art__letter">C</span>
        <span className="word-art__letter">?</span>
        <span className="word-art__letter word-art__letter--accent">Aa</span>
      </div>
      <span className="game-art__art-label">FIND THE RIGHT ORDER</span>
    </div>
  );
}
