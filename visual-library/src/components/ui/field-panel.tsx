import type { ReactNode } from "react";

type FieldPanelProps = {
  children: ReactNode;
  className?: string;
  tone?: "paper" | "quiet" | "dark";
};

export function FieldPanel({ children, className = "", tone = "paper" }: FieldPanelProps) {
  return (
    <div className={"field-panel field-panel--" + tone + (className ? " " + className : "")}>
      {children}
    </div>
  );
}
