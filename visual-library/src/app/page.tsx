import { AtlasSection } from "@/components/atlas-section";
import { GameLibrary } from "@/components/game-library";
import { TavernSection } from "@/components/tavern-section";
import { WayfarerSection } from "@/components/wayfarer-section";
import { ActionLink } from "@/components/ui/action-link";
import { SectionEyebrow } from "@/components/ui/section-eyebrow";
import { SiteFooter } from "@/components/ui/site-footer";
import { SiteHeader } from "@/components/ui/site-header";
import { StatusIndicator } from "@/components/ui/status-indicator";

export default function HomePage() {
  return (
    <div className="site-shell" id="top">
      <div className="announcement">
        <span className="announcement__signal" aria-hidden="true" />
        <span>MOONBYTE FIELD GUIDE</span>
        <span className="announcement__divider">/</span>
        <span>THE FIRST FOUR GAMES ARE READY</span>
        <a href="#catalogue">Explore the library <span aria-hidden="true">↘</span></a>
      </div>

      <SiteHeader />

      <main id="main-content" tabIndex={-1}>
        <section className="hero" aria-labelledby="hero-title">
          <div className="hero__copy">
            <SectionEyebrow index="01" label="PIXEL WORLDS · QUICK GAMES · SHARED ADVENTURES" />
            <h1 id="hero-title">A little world.<br /><em>A lot to discover.</em></h1>
            <p className="hero__description">
              Moonbyte brings games, puzzles and a growing world into the
              Discord servers you already call home. Drop in for a quick round
              or follow a path somewhere new.
            </p>
            <div className="hero__actions">
              <ActionLink href="/#catalogue" size="large">
                Browse the arcade <span aria-hidden="true">↘</span>
              </ActionLink>
              <ActionLink href="/#atlas" variant="quiet">
                See the world <span aria-hidden="true">→</span>
              </ActionLink>
            </div>
            <div className="hero__footnote">
              <span className="hero__footnote-mark" aria-hidden="true">✳</span>
              Play at your pace. Your progress never asks you to hurry.
            </div>
          </div>

          <div
            className="hero-world"
            role="img"
            aria-label="Original pixel-inspired Moonbyte landscape with a moon, distant hills, a small lantern village and winding path."
          >
            <div className="hero-world__grid" />
            <div className="hero-world__stars" aria-hidden="true">
              <i /><i /><i /><i /><i /><i /><i />
            </div>
            <div className="hero-world__moon" aria-hidden="true">
              <span /><span /><span />
            </div>
            <div className="hero-world__orbit" aria-hidden="true" />
            <div className="hero-world__ridge hero-world__ridge--far" aria-hidden="true" />
            <div className="hero-world__ridge hero-world__ridge--near" aria-hidden="true" />
            <div className="hero-world__village" aria-hidden="true">
              <i /><i /><i /><i />
            </div>
            <div className="hero-world__path" aria-hidden="true" />
            <div className="hero-world__lantern" aria-hidden="true" />
            <div className="hero-world__frame hero-world__frame--top" aria-hidden="true" />
            <div className="hero-world__frame hero-world__frame--bottom" aria-hidden="true" />
            <div className="hero-world__caption">
              <span>WORLD INDEX / 001</span>
              <strong>THE QUIET ORBIT</strong>
              <span>HEARTHLAND · HIGH ASTRAL</span>
            </div>
            <div className="hero-world__stamp">
              <span>FIELD</span>
              <strong>01</strong>
              <span>NOTES</span>
            </div>
          </div>
        </section>

        <section className="signal-strip" aria-label="Moonbyte principles">
          <span>PLAY TOGETHER</span><b aria-hidden="true">✳</b>
          <span>DISCOVER SOMETHING NEW</span><b aria-hidden="true">✳</b>
          <span>MAKE IT YOURS</span><b aria-hidden="true">✳</b>
          <span>ALWAYS YOUR CHOICE</span>
        </section>

        <section className="facts-row" aria-label="Current Moonbyte build">
          <div className="fact">
            <strong>04</strong>
            <span>Playable games</span>
          </div>
          <div className="fact">
            <strong>05</strong>
            <span>Atlas locations</span>
          </div>
          <div className="fact">
            <strong>11</strong>
            <span>Live Tavern destinations</span>
          </div>
          <div className="fact fact--note">
            <StatusIndicator className="fact__live" label="CURRENT BUILD" />
            <span>Only implemented features are shown as playable.</span>
          </div>
        </section>

        <section className="content-section catalogue-section" id="catalogue" aria-labelledby="catalogue-title">
          <div className="section-heading">
            <div>
              <SectionEyebrow index="02" label="THE COMMON TABLES" />
              <h2 id="catalogue-title">Pick your <em>next move.</em></h2>
            </div>
            <p>
              Four games are playable in the bot today. Find one that fits the
              moment, learn the controls, then take the command back to Discord.
            </p>
          </div>
          <GameLibrary />
        </section>

        <AtlasSection />
        <TavernSection />
        <WayfarerSection />

        <section className="principles-section" id="principles" aria-labelledby="principles-title">
          <div className="principles-intro">
            <SectionEyebrow index="06" label="BUILT AROUND PEOPLE" />
            <h2 id="principles-title">A game should fit <em>your life.</em></h2>
            <p>
              The world can be deep without asking for all your time. The best
              reason to come back is that there is something genuinely good to play.
            </p>
          </div>
          <div className="principle-list">
            <article>
              <span className="principle-list__number">01</span>
              <div><h3>Play, don’t grind</h3><p>Quick rounds and longer journeys both have a place. Progress never decays when you leave.</p></div>
              <span className="principle-list__symbol" aria-hidden="true">↗</span>
            </article>
            <article>
              <span className="principle-list__number">02</span>
              <div><h3>Make every connection count</h3><p>Games, the Atlas, Chronicle and Wayfarer are designed to share meaningful outcomes.</p></div>
              <span className="principle-list__symbol" aria-hidden="true">⌁</span>
            </article>
            <article>
              <span className="principle-list__number">03</span>
              <div><h3>Know what is real</h3><p>Live features are labelled clearly. Planned systems stay on the roadmap until they are built.</p></div>
              <span className="principle-list__symbol" aria-hidden="true">◎</span>
            </article>
          </div>
        </section>
      </main>

      <SiteFooter />
    </div>
  );
}
