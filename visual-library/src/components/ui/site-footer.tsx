import Link from "next/link";
import { BrandMark } from "@/components/ui/brand-mark";

export function SiteFooter() {
  return (
    <footer className="site-footer">
      <Link className="brand brand--footer" href="/" aria-label="Moonbyte home">
        <BrandMark />
        <span className="brand__wordmark">moonbyte</span>
      </Link>
      <p>A little world of games, right inside Discord.</p>
      <div className="footer-links">
        <a href="https://github.com/VireonNoctis/Moonbyte" rel="noreferrer" target="_blank">
          Repository <span aria-hidden="true">↗</span>
        </a>
        <a
          href="https://github.com/VireonNoctis/Moonbyte/blob/main/docs/MOONBYTE_FINAL_VISION.md"
          rel="noreferrer"
          target="_blank"
        >
          Product vision <span aria-hidden="true">↗</span>
        </a>
      </div>
      <span className="footer-note">ORIGINAL WORLDS · HONEST PROGRESS · NO PRESSURE</span>
    </footer>
  );
}
