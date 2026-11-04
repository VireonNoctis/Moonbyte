import Link from "next/link";
import { BrandMark } from "@/components/ui/brand-mark";

type SiteSection = "arcade" | "atlas" | "tavern" | "wayfarer";

type SiteHeaderProps = {
  active?: SiteSection;
  actionLabel?: string;
};

const navigation: { id: SiteSection; label: string; href: string }[] = [
  { id: "arcade", label: "Arcade", href: "/#catalogue" },
  { id: "atlas", label: "Atlas", href: "/#atlas" },
  { id: "tavern", label: "Tavern", href: "/#tavern" },
  { id: "wayfarer", label: "Wayfarer", href: "/#wayfarer" },
];

export function SiteHeader({ active, actionLabel = "Find a game" }: SiteHeaderProps) {
  return (
    <header className="site-header">
      <Link className="brand" href="/" aria-label="Moonbyte home">
        <BrandMark />
        <span className="brand__wordmark">moonbyte</span>
        <span className="brand__edition">FIELD GUIDE 01</span>
      </Link>
      <nav className="primary-nav" aria-label="Main navigation">
        {navigation.map((item) => (
          <Link
            aria-current={active === item.id ? "location" : undefined}
            href={item.href}
            key={item.id}
          >
            {item.label}
          </Link>
        ))}
      </nav>
      <Link className="header-action" href="/#catalogue">
        {actionLabel} <span aria-hidden="true">↗</span>
      </Link>
    </header>
  );
}
