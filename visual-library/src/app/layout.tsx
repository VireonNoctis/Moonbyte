import type { Metadata, Viewport } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Moonbyte — Play a little differently",
  description: "Explore Moonbyte's current Discord game library: quick strategy, memory, and wordplay sessions.",
  applicationName: "Moonbyte Visual Library",
  robots: { index: false, follow: false },
};

export const viewport: Viewport = {
  themeColor: "#10130f",
  colorScheme: "dark",
};

export default function RootLayout({ children }: Readonly<{ children: React.ReactNode }>) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
