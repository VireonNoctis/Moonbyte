import { ActionLink } from "@/components/ui/action-link";
import { FieldPanel } from "@/components/ui/field-panel";
import { SectionEyebrow } from "@/components/ui/section-eyebrow";
import { SiteFooter } from "@/components/ui/site-footer";
import { SiteHeader } from "@/components/ui/site-header";

export default function NotFound() {
  return (
    <div className="site-shell not-found-shell">
      <SiteHeader />
      <main className="not-found" id="main-content" tabIndex={-1}>
        <FieldPanel className="not-found__panel" tone="paper">
          <span className="not-found__code" aria-hidden="true">404</span>
          <SectionEyebrow index="—" label="NO MAP MARKER HERE" />
          <h1>This page isn’t <em>on the map.</em></h1>
          <p>
            That route doesn’t belong to the current Field Guide. The playable
            catalogue is the best place to choose your next stop.
          </p>
          <div className="not-found__actions">
            <ActionLink href="/#catalogue" size="large">
              Back to the arcade <span aria-hidden="true">↘</span>
            </ActionLink>
            <ActionLink href="/" variant="quiet">
              Return home <span aria-hidden="true">→</span>
            </ActionLink>
          </div>
        </FieldPanel>
        <div className="not-found__coordinate" aria-hidden="true">
          MOONBYTE FIELD GUIDE / ROUTE NOT FOUND / 00—00
        </div>
      </main>
      <SiteFooter />
    </div>
  );
}
