"use client";

import { useState, type ReactNode } from "react";

type CopyCommandButtonProps = {
  value: string;
  children: ReactNode;
  className?: string;
  copiedLabel?: string;
};

export function CopyCommandButton({
  value,
  children,
  className = "copy-button",
  copiedLabel = "Copied",
}: CopyCommandButtonProps) {
  const [status, setStatus] = useState<"idle" | "copied" | "unavailable">("idle");

  async function copyValue() {
    if (!navigator.clipboard?.writeText) {
      setStatus("unavailable");
      return;
    }

    try {
      await navigator.clipboard.writeText(value);
      setStatus("copied");
    } catch {
      setStatus("unavailable");
    }
  }

  return (
    <button
      className={className}
      data-copy-state={status}
      onClick={copyValue}
      type="button"
      aria-label={status === "idle" ? String(value) + " — copy command" : undefined}
    >
      <span aria-live="polite">
        {status === "copied" ? copiedLabel : status === "unavailable" ? "Copy unavailable" : children}
      </span>
      <span className="copy-button__symbol" aria-hidden="true">
        {status === "copied" ? "✓" : "↗"}
      </span>
    </button>
  );
}
