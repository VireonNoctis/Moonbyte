type StatusIndicatorProps = {
  label: string;
  className?: string;
};

export function StatusIndicator({ label, className = "" }: StatusIndicatorProps) {
  return (
    <span className={"status-indicator" + (className ? " " + className : "")}>
      <i aria-hidden="true" />
      {label}
    </span>
  );
}
