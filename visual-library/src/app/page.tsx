import { GameLibrary } from "@/components/game-library";

export default function HomePage() {
  return (
    <main className="site-shell">
      <div className="announcement">
        <span className="announcement__signal" aria-hidden="true" />
        <p>THE ARCADE IS OPEN <span>·</span> FIRST PLAYABLE BUILD</p>
        <a href="#library">Explore games <span aria-hidden="true">↘</span></a>
      </div>

      <header className="site-header">
        <a className="brand" href="#" aria-label="Moonbyte home">
          <span className="brand__mark" aria-hidden="true">
            <span /><span /><span /><span />
          </span>
          <span className="brand__wordmark">moonbyte<span>®</span></span>
        </a>
        <nav className="header-nav" aria-label="Main navigation">
          <a className="header-nav__active" href="#library">Library</a>
          <a href="#how-it-works">How it works</a>
          <a href="https://github.com/VireonNoctis/Moonbyte" rel="noreferrer" target="_blank">Source ↗</a>
        </nav>
        <a className="header-cta" href="#library">Pick a game <span aria-hidden="true">↗</span></a>
      </header>

      <section className="hero" aria-labelledby="hero-title">
        <div className="hero__copy">
          <div className="eyebrow"><span>01</span> YOUR SERVER, YOUR PLAYGROUND</div>
          <h1 id="hero-title">Small games.<br /><em>Big little</em> moments.</h1>
          <p className="hero__description">
            A colourful pocket arcade for the places you already hang out.
            Challenge a friend, sharpen your instincts, or take a tiny break.
          </p>
          <div className="hero__actions">
            <a className="button button--primary button--large" href="#library">Find a game <span aria-hidden="true">↘</span></a>
            <a className="text-link" href="#how-it-works">How Moonbyte works <span aria-hidden="true">→</span></a>
          </div>
          <div className="hero__note"><span aria-hidden="true">✳</span> No streak pressure. Drop in, drop out.</div>
        </div>
        <div className="hero-art" aria-label="Abstract original pixel-art moon and landscape">
          <div className="hero-art__stars" aria-hidden="true">✦　 ·　 ✧　　✦　 ·</div>
          <div className="hero-art__moon" aria-hidden="true"><span /></div>
          <div className="hero-art__planet" aria-hidden="true" />
          <div className="hero-art__mountain hero-art__mountain--back" aria-hidden="true" />
          <div className="hero-art__mountain hero-art__mountain--front" aria-hidden="true" />
          <div className="hero-art__pixel hero-art__pixel--one" aria-hidden="true" />
          <div className="hero-art__pixel hero-art__pixel--two" aria-hidden="true" />
          <div className="hero-art__label"><span>WORLD 001</span><strong>THE QUIET ORBIT</strong><span>EST. IN THE MOONLIGHT</span></div>
          <div className="hero-art__corner hero-art__corner--tl" aria-hidden="true" />
          <div className="hero-art__corner hero-art__corner--br" aria-hidden="true" />
        </div>
        <div className="hero__side-label" aria-hidden="true">PLAY IS A PLACE, NOT A CHORE</div>
      </section>

      <section className="ticker" aria-label="Moonbyte principles">
        <span>PLAY TOGETHER</span><b>✳</b><span>MAKE A LITTLE PROGRESS</span><b>✳</b><span>KEEP IT HUMAN</span><b>✳</b><span>ALWAYS YOUR CHOICE</span>
      </section>

      <section className="library-section" id="library" aria-labelledby="library-title">
        <div className="section-heading">
          <div>
            <div className="eyebrow"><span>02</span> THE CURRENT LINE-UP</div>
            <h2 id="library-title">Choose your <em>next move.</em></h2>
          </div>
          <p>Four games are playable in the Discord bot today. This library shows what is implemented—not a mock catalogue of future promises.</p>
        </div>
        <GameLibrary />
      </section>

      <section className="how-section" id="how-it-works" aria-labelledby="how-title">
        <div className="how-section__intro">
          <div className="eyebrow"><span>03</span> MADE FOR THE MOMENT</div>
          <h2 id="how-title">A game should fit <em>your life.</em></h2>
          <p>Moonbyte is built around meaningful play, not pressure to keep up.</p>
        </div>
        <div className="principle-list">
          <article><span className="principle-list__number">01</span><div><h3>Start in seconds</h3><p>Choose a game and copy its command into a server where Moonbyte is installed.</p></div><span className="principle-list__icon" aria-hidden="true">↗</span></article>
          <article><span className="principle-list__number">02</span><div><h3>Play at your pace</h3><p>Sessions are bounded. You can leave without losing a streak or being nagged.</p></div><span className="principle-list__icon" aria-hidden="true">◷</span></article>
          <article><span className="principle-list__number">03</span><div><h3>Know what is real</h3><p>Rules, controls, and implementation status are visible. Planned features stay labelled planned.</p></div><span className="principle-list__icon" aria-hidden="true">◎</span></article>
        </div>
      </section>

      <footer className="site-footer">
        <a className="brand" href="#" aria-label="Moonbyte home">
          <span className="brand__mark" aria-hidden="true"><span /><span /><span /><span /></span>
          <span className="brand__wordmark">moonbyte<span>®</span></span>
        </a>
        <p>A little world of games, right inside Discord.</p>
        <div><a href="https://github.com/VireonNoctis/Moonbyte" rel="noreferrer" target="_blank">Open source ↗</a><span>© Moonbyte · Built for people, not streaks.</span></div>
      </footer>
    </main>
  );
}
