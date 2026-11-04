"use client";

import { useEffect, useRef, useState, type ReactNode } from "react";

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
  const timeoutRef = useRef<ReturnType<typeof window.setTimeout> | null>(null);

  useEffect(() => {
    return () => {
      if (timeoutRef.current !== null) {
        window.clearTimeout(timeoutRef.current);
      }
    };
  }, []);

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

    if (timeoutRef.current !== null) {
      window.clearTimeout(timeoutRef.current);
    }

    timeoutRef.current = window.setTimeout(() => {
      setStatus("idle");
      timeoutRef.current = null;
    }, 1800);
  }

  return (
    <button
      className={className}
      data-copy-state={status}
      onClick={copyValue}
      type="button"
      aria-label={status === "idle" ? String(value) + " — copy command" : undefined}
    >
      <span>{status === "copied" ? copiedLabel : status === "unavailable" ? "Copy unavailable" : children}</span>
      <span className="copy-button__symbol" aria-hidden="true">
        {status === "copied" ? "✓" : "↗"}
      </span>
    </button>
  );
}
