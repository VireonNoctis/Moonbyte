import { CopyCommandButton } from "@/components/copy-command";

const objectives = [
  {
    id: "play_one_game",
    title: "Warm Up the Arcade",
    description: "Complete one game.",
    reward: 5,
    mark: "01",
    tone: "mint",
  },
  {
    id: "play_three_games",
    title: "A Few More Rounds",
    description: "Complete three games.",
    reward: 8,
    mark: "02",
    tone: "sun",
  },
  {
    id: "win_one_game",
    title: "Find Your Rhythm",
    description: "Win one game.",
    reward: 8,
    mark: "03",
    tone: "coral",
  },
  {
    id: "play_two_game_types",
    title: "Try Another Table",
    description: "Complete two different game types.",
    reward: 10,
    mark: "04",
    tone: "lilac",
  },
  {
    id: "discover_one_place",
    title: "A New Corner of the Map",
    description: "Discover one new Atlas location.",
    reward: 7,
    mark: "05",
    tone: "blue",
  },
] as const;

export function WayfarerSection() {
  return (
    <section className="wayfarer-section content-section" id="wayfarer" aria-labelledby="wayfarer-title">
      <div className="wayfarer-intro">
        <div className="eyebrow"><span className="eyebrow__index">05</span><span>THE WAYFARER THREAD</span></div>
        <h2 id="wayfarer-title">Progress that <em>stays yours.</em></h2>
        <p>
          Optional goals connect the arcade to the wider world. Journey days
          count up, companion stages stay earned, and missing a day never takes
          anything away.
        </p>

        <div className="wayfarer-facts">
          <div><strong>03</strong><span>objectives assigned each day</span></div>
          <div><strong>+</strong><span>cumulative journey days</span></div>
          <div><strong>0</strong><span>missed-day penalties</span></div>
        </div>

        <div className="wayfarer-command">
          <span>OPEN THE BOARD IN DISCORD</span>
          <CopyCommandButton value=".quests" className="copy-button copy-button--dark">
            Copy .quests
          </CopyCommandButton>
        </div>
      </div>

      <div className="wayfarer-board">
        <div className="wayfarer-board__heading">
          <div>
            <span>THE OBJECTIVE CATALOGUE</span>
            <h3>Small steps, fixed rewards.</h3>
          </div>
          <span className="wayfarer-board__count">05 / 05</span>
        </div>

        <div className="wayfarer-grid">
          {objectives.map((objective) => (
            <article className={"wayfarer-card wayfarer-card--" + objective.tone} key={objective.id}>
              <div className="wayfarer-card__topline">
                <span>{objective.mark}</span>
                <span>OPTIONAL QUEST</span>
              </div>
              <h4>{objective.title}</h4>
              <p>{objective.description}</p>
              <div className="wayfarer-card__reward">
                <span aria-hidden="true">✳</span>
                <strong>{objective.reward}</strong>
                <span>Moonbits</span>
              </div>
            </article>
          ))}
        </div>

        <div className="wayfarer-board__foot">
          <span>THREE ARE ASSIGNED PER PLAYER / DAY</span>
          <span>FIXED REWARDS · NO ATTENDANCE PENALTY</span>
        </div>
      </div>

      <p className="wayfarer-disclaimer">
        This page shows the fixed objective catalogue only. Your daily selection, accepted quests
        and personal progress remain private inside Discord.
      </p>
    </section>
  );
}
