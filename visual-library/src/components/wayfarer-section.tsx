const objectives = [
  {
    id: "play_one_game",
    title: "Warm Up the Arcade",
    description: "Complete one game today.",
    reward: 5,
    mark: "01",
  },
  {
    id: "play_three_games",
    title: "A Few More Rounds",
    description: "Complete three games today.",
    reward: 8,
    mark: "02",
  },
  {
    id: "win_one_game",
    title: "Find Your Rhythm",
    description: "Win one game today.",
    reward: 8,
    mark: "03",
  },
  {
    id: "play_two_game_types",
    title: "Try Another Table",
    description: "Complete two different game types.",
    reward: 10,
    mark: "04",
  },
  {
    id: "discover_one_place",
    title: "A New Corner of the Map",
    description: "Discover one new Atlas location.",
    reward: 7,
    mark: "05",
  },
] as const;

export function WayfarerSection() {
  return (
    <section className="wayfarer-section" id="wayfarer" aria-labelledby="wayfarer-title">
      <div className="wayfarer-intro">
        <div className="eyebrow"><span>03</span> THE WAYFARER THREAD</div>
        <h2 id="wayfarer-title">A journey that <em>keeps its meaning.</em></h2>
        <p>
          A few optional goals give each visit a direction. The progress you earn
          stays yours; Moonbyte never takes it away for missing a day.
        </p>
        <div className="wayfarer-facts">
          <div>
            <span className="wayfarer-facts__number">03</span>
            <span>objectives on each daily board</span>
          </div>
          <div>
            <span className="wayfarer-facts__number">+</span>
            <span>cumulative journey days</span>
          </div>
          <div>
            <span className="wayfarer-facts__number">0</span>
            <span>missed-day penalties</span>
          </div>
        </div>
        <div className="wayfarer-command">
          <span>OPEN THE BOARD IN DISCORD</span>
          <code>.quests</code>
        </div>
      </div>

      <div className="wayfarer-catalog">
        <div className="wayfarer-catalog__heading">
          <div>
            <span className="wayfarer-catalog__kicker">THE OBJECTIVE CATALOGUE</span>
            <h3>Small steps, fixed rewards.</h3>
          </div>
          <span className="wayfarer-catalog__count">05 / 05</span>
        </div>
        <div className="wayfarer-grid">
          {objectives.map((objective) => (
            <article className="wayfarer-card" key={objective.id}>
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
        <div className="wayfarer-catalog__foot">
          <span>Three objectives are selected for each player each day.</span>
          <span>Rewards are fixed · Progress is never reset</span>
        </div>
      </div>
    </section>
  );
}
