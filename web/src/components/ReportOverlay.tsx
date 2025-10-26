import { useSelector } from "react-redux";
import { type RootState } from "./Store";
import type { Incident } from "../structs/Incident";
import { Distance } from "../utils/Distance";
import React, { useEffect } from "react";
import { DurationAgo } from "../utils/FormatTime";
import { GetIncidents } from "../utils/GetIncidents";

export default function ReportOverlay() {
  const [userLocation, setUserLocation] = React.useState<[number, number]>([
    34.0549, -118.2426,
  ]);
  const [incidents, setIncidents] = React.useState<Array<Incident>>([]);

  React.useEffect(() => {
    GetIncidents()
      .then((r) => setIncidents(r))
      .catch((e) => console.log(e));
    console.log(incidents.length);
  }, []);
  const activeMarkerId = useSelector((state: RootState) => state.map.value);

  useEffect(() => {
    navigator.geolocation.getCurrentPosition((position) => {
      setUserLocation([position.coords.latitude, position.coords.longitude]);
    });
  }, []);

  const activeMarker: Incident | undefined = incidents.find(
    (a: Incident) => a.incident_id === activeMarkerId,
  );
  if (activeMarker === undefined) {
    return <></>;
  }

  return (
    <div
      className="report-info-wrapper flexbox"
      style={{
        position: "relative",
        zIndex: 9,
        left: 0,
        margin: "0 auto",
        width: "100%",
        textAlign: "center",
        maxWidth: "90vw",
      }}
    >
      <div
        style={{
          bottom: 10,
          position: "absolute",
          padding: "1em 3em",
          background: "black",
          color: "white",
          fontSize: ".85rem",
          borderRadius: "1em",
          fontFamily: "Lato",
          textAlign: "left",
        }}
      >
        <h5 style={{ color: "#9CA3AF", fontSize: "1.1em" }}>
          {Distance(
            activeMarker.lat,
            activeMarker.long,
            userLocation[0],
            userLocation[1],
          )}
          {"mi — "}
          {activeMarker.address.split(",")[0]}
        </h5>
        <h1>{activeMarker.title}</h1>
        <h2 style={{ fontSize: "1.3em" }}>
          Reported {DurationAgo(activeMarker.time_unix)}
        </h2>
      </div>
    </div>
  );
}
