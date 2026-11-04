import { CopyCommandButton } from "@/components/copy-command";

const destinations = [
  { group: "PLAY", title: "The Common Tables", description: "Browse the four live games and start a solo session or lobby.", command: "games", mark: "01" },
  { group: "YOUR CORNER", title: "Your Place", description: "Open your private player profile and saved preferences.", command: "profile", mark: "02" },
  { group: "YOUR CORNER", title: "Your Record", description: "Review your completed games, wins and draws.", command: "stats", mark: "03" },
  { group: "YOUR CORNER", title: "The Ledger", description: "View earned-only Moonbits and first-completion milestones.", command: "wallet", mark: "04" },
  { group: "YOUR CORNER", title: "The Gallery", description: "Browse five achievement milestones earned through play.", command: "achievements", mark: "05" },
  { group: "DISCOVER", title: "The Atlas", description: "See your current place, charted discoveries and direct routes.", command: "atlas", mark: "06" },
  { group: "DISCOVER", title: "The Chronicle", description: "Read private lore pages recorded by first-time discoveries.", command: "chronicle", mark: "07" },
  { group: "JOURNEY", title: "The Quest Board", description: "Accept optional daily goals and claim fixed rewards.", command: "quests", mark: "08" },
  { group: "JOURNEY", title: "The Wayfarer Thread", description: "Review cumulative journey days and companion milestones.", command: "wayfarer", mark: "09" },
  { group: "JOURNEY", title: "The Companion Journal", description: "See Mallow’s cosmetic companion stage.", command: "companion", mark: "10" },
  { group: "TOGETHER", title: "The Rivalry Hall", description: "Manage mutual invitations and private head-to-head records.", command: "rivals", mark: "11" },
] as const;

export function TavernSection() {
  return (
    <section className="tavern-section content-section" id="tavern" aria-labelledby="tavern-title">
      <div className="section-heading">
        <div>
          <div className="eyebrow"><span className="eyebrow__index">04</span><span>THE LANTERN AND LADLE</span></div>
          <h2 id="tavern-title">One door. <em>Eleven destinations.</em></h2>
        </div>
        <p>
          The Tavern is the home screen for the features that are live today.
          Each stop leads to a real command; browsing the Wayfinder never changes saved progress.
        </p>
      </div>

      <div className="tavern-board">
        <div className="tavern-board__header">
          <div className="tavern-board__seal" aria-hidden="true">✳</div>
          <div>
            <span>THE WAYFINDER / LIVE ROUTES</span>
            <strong>Choose where to go next.</strong>
          </div>
          <span className="tavern-board__count">11 OPEN</span>
        </div>

        <div className="destination-grid">
          {destinations.map((destination) => (
            <article className="destination-card" key={destination.command}>
              <div className="destination-card__top">
                <span>{destination.group}</span>
                <span>{destination.mark}</span>
              </div>
              <h3>{destination.title}</h3>
              <p>{destination.description}</p>
              <div className="destination-card__command">
                <code>.{destination.command}</code>
                <CopyCommandButton
                  className="copy-button copy-button--icon"
                  value={"." + destination.command}
                  copiedLabel="Copied"
                >
                  Copy
                </CopyCommandButton>
              </div>
            </article>
          ))}
        </div>

        <div className="tavern-board__footer">
          <span>DEFAULT PREFIX / .</span>
          <span>SERVER MODERATORS CAN SET A GUILD PREFIX</span>
          <span>THE WAYFINDER ONLY LISTS IMPLEMENTED COMMANDS</span>
        </div>
      </div>
    </section>
  );
}
