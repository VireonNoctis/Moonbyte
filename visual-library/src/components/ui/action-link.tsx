import Link from "next/link";
import type { ReactNode } from "react";

type ActionLinkProps = {
  href: string;
  children: ReactNode;
  variant?: "primary" | "secondary" | "quiet";
  size?: "default" | "large";
  className?: string;
};

export function ActionLink({
  href,
  children,
  variant = "primary",
  size = "default",
  className = "",
}: ActionLinkProps) {
  const baseClass = variant === "quiet" ? "text-link" : "button button--" + variant;
  const sizeClass = size === "large" && variant !== "quiet" ? " button--large" : "";
  const extraClass = className ? " " + className : "";

  return (
    <Link className={baseClass + sizeClass + extraClass} href={href}>
      {children}
    </Link>
  );
}
