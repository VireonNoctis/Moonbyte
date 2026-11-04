type SectionEyebrowProps = {
  index: string;
  label: string;
  light?: boolean;
};

export function SectionEyebrow({ index, label, light = false }: SectionEyebrowProps) {
  return (
    <div className={"eyebrow" + (light ? " eyebrow--light" : "")}>
      <span className="eyebrow__index">{index}</span>
      <span>{label}</span>
    </div>
  );
}
