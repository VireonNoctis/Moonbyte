import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import { CopyCommandButton } from "@/components/copy-command";
import { GameArtwork } from "@/components/game-artwork";
import { GuideFact } from "@/components/ui/guide-fact";
import { SectionEyebrow } from "@/components/ui/section-eyebrow";
import { games } from "@/lib/catalog";

type GamePageProps = {
  params: Promise<{ slug: string }>;
};

export const dynamicParams = false;

export function generateStaticParams() {
  return games.map((game) => ({ slug: game.id }));
}

export async function generateMetadata({ params }: GamePageProps): Promise<Metadata> {
  const { slug } = await params;
  const game = games.find((item) => item.id === slug);

  if (!game) {
    return { title: "Game not found — Moonbyte" };
  }

  return {
    title: game.name + " — Moonbyte Field Guide",
    description: game.description + " Learn the current controls and take the command to Discord.",
  };
}

export default async function GamePage({ params }: GamePageProps) {
  const { slug } = await params;
  const gameIndex = games.findIndex((item) => item.id === slug);
  const game = games[gameIndex];

  if (!game) {
    notFound();
  }

  const relatedGames = games.filter((item) => item.id !== game.id);

  return (
    <div className="site-shell game-guide-shell">
      <div className="game-guide-utility">
        <Link href="/#catalogue"><span aria-hidden="true">←</span> Back to the Common Tables</Link>
        <span>FIELD GUIDE / GAME 0{gameIndex + 1}</span>
      </div>

      <header className="site-header">
        <Link className="brand" href="/" aria-label="Moonbyte home">
          <span className="brand-mark" aria-hidden="true"><i /><i /><i /><i /></span>
          <span className="brand__wordmark">moonbyte</span>
          <span className="brand__edition">FIELD GUIDE 01</span>
        </Link>
        <nav className="primary-nav" aria-label="Main navigation">
          <Link href="/#catalogue" aria-current="page">Arcade</Link>
          <Link href="/#atlas">Atlas</Link>
          <Link href="/#tavern">Tavern</Link>
          <Link href="/#wayfarer">Wayfarer</Link>
        </nav>
        <Link className="header-action" href="/#catalogue">All games <span aria-hidden="true">↗</span></Link>
      </header>

      <main className="game-guide" id="top">
        <div className="game-guide__breadcrumb">
          <Link href="/#catalogue">THE COMMON TABLES</Link>
          <span aria-hidden="true">/</span>
          <span>{game.category.toUpperCase()}</span>
          <span aria-hidden="true">/</span>
          <span>{game.name.toUpperCase()}</span>
        </div>

        <section className="game-guide__hero" aria-labelledby="game-title">
          <div className={"game-guide__art game-guide__art--" + game.palette}>
            <div className="game-guide__art-topline">
              <span>MOONBYTE / {String(gameIndex + 1).padStart(2, "0")}</span>
              <span className="playable-label"><i aria-hidden="true" />LIVE IN DISCORD</span>
            </div>
            <GameArtwork game={game} featured />
            <div className="game-guide__art-footer">
              <span>THE CURRENT LINE-UP</span>
              <span>ORIGINAL FIELD ART</span>
            </div>
          </div>

          <div className="game-guide__intro">
            <SectionEyebrow index={"0" + String(gameIndex + 2)} label="THE COMMON TABLES" />
            <div className="game-guide__category-row">
              <span className="game-guide__category">{game.category}</span>
              <span className="game-guide__status"><i /> PLAYABLE NOW</span>
            </div>
            <h1 id="game-title">{game.name}<span>.</span></h1>
            <p className="game-guide__description">{game.description}</p>
            <div className="game-guide__actions">
              <CopyCommandButton value={game.command} className="copy-button copy-button--primary game-guide__copy">
                Copy play command
              </CopyCommandButton>
              <span>THEN RUN IT IN YOUR DISCORD SERVER</span>
            </div>
            <dl className="game-guide__facts">
              <GuideFact label="PLAYERS" value={game.players} />
              <GuideFact label="SESSION" value={game.sessionType} />
              <GuideFact label="EST. TIME" value={game.duration} />
            </dl>
          </div>
        </section>

        <section className="game-guide__start" aria-labelledby="start-title">
          <div className="game-guide__section-intro">
            <SectionEyebrow index="01" label="BEFORE YOU PLAY" />
            <h2 id="start-title">A clear start.<br /><em>No hidden steps.</em></h2>
            <p>Moonbyte handles the game session in Discord. This page is your field guide; copying a command does not start a match or change saved progress.</p>
          </div>

          <div className="game-guide__instruction-card">
            <article>
              <span className="game-guide__step">01</span>
              <div>
                <h3>Open a Discord channel</h3>
                <p>Choose a server channel where Moonbyte is available and your server’s command settings allow play.</p>
              </div>
            </article>
            <article>
              <span className="game-guide__step">02</span>
              <div>
                <h3>{game.sessionType === "Solo session" ? "Start your solo session" : "Open a two-player lobby"}</h3>
                <p>{game.sessionType === "Solo session" ? "Run the command below to begin your own session." : "Run the command below to create a lobby, then have the other player join that session."}</p>
              </div>
            </article>
            <article>
              <span className="game-guide__step">03</span>
              <div>
                <h3>Make your move</h3>
                <p>{game.controls}</p>
              </div>
            </article>
            <div className="game-guide__command-row">
              <code>{game.command}</code>
              <CopyCommandButton value={game.command} className="copy-button copy-button--dark">
                Copy command
              </CopyCommandButton>
            </div>
          </div>
        </section>

        <section className="game-guide__build-note" aria-labelledby="build-note-title">
          <div>
            <SectionEyebrow index="02" label="CURRENT BUILD" />
            <h2 id="build-note-title">What’s in this version.</h2>
          </div>
          <div className="game-guide__note-content">
            {game.implementationNote ? (
              <p>{game.implementationNote}</p>
            ) : (
              <p>This guide reflects the live Rust game and session catalogue. Game rules, player actions and results are handled by the bot, not by this website.</p>
            )}
            <span className="game-guide__truth-label"><i /> MATCHED TO THE CURRENT CATALOGUE</span>
          </div>
        </section>

        <section className="game-guide__related" aria-labelledby="related-title">
          <div className="game-guide__related-heading">
            <div>
              <SectionEyebrow index="03" label="KEEP EXPLORING" />
              <h2 id="related-title">Another table <em>is open.</em></h2>
            </div>
            <Link href="/#catalogue">View all games <span aria-hidden="true">↗</span></Link>
          </div>
          <div className="game-guide__related-grid">
            {relatedGames.map((related, index) => (
              <Link className={"game-guide__related-card game-guide__related-card--" + related.palette} href={"/games/" + related.id} key={related.id}>
                <span>0{index + 1} / {related.category.toUpperCase()}</span>
                <strong>{related.name}</strong>
                <span className="game-guide__related-arrow" aria-hidden="true">↗</span>
              </Link>
            ))}
          </div>
        </section>
      </main>

      <footer className="site-footer">
        <Link className="brand brand--footer" href="/" aria-label="Moonbyte home">
          <span className="brand-mark" aria-hidden="true"><i /><i /><i /><i /></span>
          <span className="brand__wordmark">moonbyte</span>
        </Link>
        <p>A little world of games, right inside Discord.</p>
        <div className="footer-links">
          <a href="https://github.com/VireonNoctis/Moonbyte" rel="noreferrer" target="_blank">Repository <span aria-hidden="true">↗</span></a>
          <Link href="/#catalogue">All games <span aria-hidden="true">↗</span></Link>
        </div>
        <span className="footer-note">ORIGINAL WORLDS · HONEST PROGRESS · NO PRESSURE</span>
      </footer>
    </div>
  );
}
