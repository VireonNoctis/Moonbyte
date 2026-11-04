type GuideFactProps = {
  label: string;
  value: string;
};

export function GuideFact({ label, value }: GuideFactProps) {
  return (
    <div className="game-guide__fact">
      <dt>{label}</dt>
      <dd>{value}</dd>
    </div>
  );
}
