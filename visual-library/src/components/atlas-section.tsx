"use client";

import { useState } from "react";
import { CopyCommandButton } from "@/components/copy-command";

type AtlasPlace = {
  id: string;
  name: string;
  region: string;
  description: string;
  chronicle: string;
  routesTo: string[];
  x: number;
  y: number;
  mark: string;
};

const places: AtlasPlace[] = [
  {
    id: "lantern-square",
    name: "Lantern Square",
    region: "Hearthland",
    description: "The warm-lit meeting place where the known roads begin.",
    chronicle: "The First Light",
    routesTo: ["mosslight-grove", "glasswater-pier"],
    x: 17,
    y: 72,
    mark: "01",
  },
  {
    id: "mosslight-grove",
    name: "Mosslight Grove",
    region: "Verdant Reach",
    description: "A quiet woodland path threaded with old stones and soft green light.",
    chronicle: "The Stones That Remember",
    routesTo: ["lantern-square", "ember-archive", "starfall-observatory"],
    x: 42,
    y: 27,
    mark: "02",
  },
  {
    id: "glasswater-pier",
    name: "Glasswater Pier",
    region: "Western Coast",
    description: "A calm shoreline of clear water, weathered maps and distant sails.",
    chronicle: "The Tidekeeper’s Measure",
    routesTo: ["lantern-square", "starfall-observatory"],
    x: 45,
    y: 80,
    mark: "03",
  },
  {
    id: "ember-archive",
    name: "The Ember Archive",
    region: "Cinder Vale",
    description: "A small archive preserving the stories of the world's old roads.",
    chronicle: "A Page Kept Warm",
    routesTo: ["mosslight-grove", "starfall-observatory"],
    x: 72,
    y: 25,
    mark: "04",
  },
  {
    id: "starfall-observatory",
    name: "Starfall Observatory",
    region: "High Astral",
    description: "A hilltop observatory with a clear view of the connected lands.",
    chronicle: "A Map for Returning",
    routesTo: ["mosslight-grove", "glasswater-pier", "ember-archive"],
    x: 85,
    y: 68,
    mark: "05",
  },
];

const routeLines = [
  "M 17 72 L 42 27",
  "M 17 72 L 45 80",
  "M 42 27 L 72 25",
  "M 42 27 L 85 68",
  "M 45 80 L 85 68",
  "M 72 25 L 85 68",
];

export function AtlasSection() {
  const [selectedId, setSelectedId] = useState("lantern-square");
  const selectedPlace = places.find((place) => place.id === selectedId) ?? places[0];
  const connectedPlaces = selectedPlace.routesTo
    .map((id) => places.find((place) => place.id === id))
    .filter((place): place is AtlasPlace => Boolean(place));

  return (
    <section className="atlas-section" id="atlas" aria-labelledby="atlas-title">
      <div className="atlas-section__heading">
        <div>
          <div className="eyebrow eyebrow--light"><span className="eyebrow__index">03</span><span>THE WORLD INDEX</span></div>
          <h2 id="atlas-title">Five places.<br /><em>One connected map.</em></h2>
        </div>
        <p>
          The first Atlas is small by design. Every road connects to a real
          location, and every first discovery adds a page to your private Chronicle.
        </p>
      </div>

      <div className="atlas-layout">
        <div className="atlas-map-wrap">
          <div className="atlas-map__topline">
            <span>HEARTHLAND SURVEY / 001</span>
            <span>SELECT A WAYPOINT</span>
          </div>
          <div className="atlas-map" aria-label="Interactive route map of the five implemented Atlas locations">
            <div className="atlas-map__texture" aria-hidden="true" />
            <svg className="atlas-map__routes" viewBox="0 0 100 100" preserveAspectRatio="none" aria-hidden="true">
              {routeLines.map((line) => <path d={line} key={line} />)}
            </svg>

            {places.map((place) => (
              <button
                aria-label={"Show " + place.name + " in " + place.region}
                aria-pressed={selectedId === place.id}
                className={"atlas-node" + (selectedId === place.id ? " atlas-node--selected" : "")}
                key={place.id}
                onClick={() => setSelectedId(place.id)}
                style={{ left: place.x + "%", top: place.y + "%" }}
                type="button"
              >
                <span className="atlas-node__pin"><i /></span>
                <span className="atlas-node__label">
                  <b>{place.mark}</b>
                  <span>{place.name}</span>
                </span>
              </button>
            ))}
          </div>
          <div className="atlas-map__legend">
            <span><i className="atlas-legend__dot atlas-legend__dot--start" />STARTING PLACE</span>
            <span><i className="atlas-legend__line" />CONNECTED ROUTE</span>
            <span>5 / 5 LOCATIONS AUTHORED</span>
          </div>
        </div>

        <aside className="atlas-detail" aria-live="polite" aria-label="Selected Atlas location">
          <div className="atlas-detail__topline">
            <span>LOCATION FILE</span>
            <span>{selectedPlace.mark} / 05</span>
          </div>
          <span className="atlas-detail__region">{selectedPlace.region}</span>
          <h3>{selectedPlace.name}</h3>
          <p>{selectedPlace.description}</p>

          <div className="atlas-detail__chronicle">
            <span className="atlas-detail__label">PRIVATE CHRONICLE PAGE</span>
            <strong>{selectedPlace.chronicle}</strong>
            <span>Unlocked by a first discovery in the live bot.</span>
          </div>

          <div className="atlas-detail__routes">
            <span className="atlas-detail__label">DIRECTLY CONNECTED TO</span>
            {connectedPlaces.map((place) => (
              <button key={place.id} onClick={() => setSelectedId(place.id)} type="button">
                <span>{place.name}</span><span aria-hidden="true">↗</span>
              </button>
            ))}
          </div>

          <CopyCommandButton value=".atlas" className="copy-button copy-button--map">
            View your private map
          </CopyCommandButton>
        </aside>
      </div>

      <p className="atlas-disclaimer">
        This is the authored world catalogue, not a player’s private progress. Your current
        location and discoveries are shown by the .atlas command inside Discord.
      </p>
    </section>
  );
}
