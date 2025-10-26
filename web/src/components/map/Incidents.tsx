import React from "react";
import L from "leaflet";
import { Marker, useMapEvents } from "react-leaflet";
//import { IncidentPopup } from "./IncidentPopup";
import { type Incident } from "../../structs/Incident";
import { useDispatch, useSelector } from "react-redux";
import { setActiveMarker } from "../Store";
import { type RootState } from "../Store";
import { GetIncidents } from "../../utils/GetIncidents";

export function Incidents() {
  const [incidents, setIncidents] = React.useState<Array<Incident>>([]);
  const dispatch = useDispatch();
  useMapEvents({
    click() {
      dispatch(setActiveMarker(""));
    },
  });

  React.useEffect(() => {
    GetIncidents()
      .then((r) => setIncidents(r))
      .catch((e) => console.log(e));
    console.log(incidents.length);
  }, []);

  const activeMarkerId = useSelector((state: RootState) => state.map.value);

  function LevelToClass(level: number): String {
    if (level === 1) return "one";
    if (level === 2) return "two";
    if (level === 3) return "three";
    if (level === 4) return "four";
    if (level === 5) return "five";
    return "one";
  }
  return (
    <>
      {incidents.map((incident: Incident) => {
        const isActive = activeMarkerId === incident.incident_id;
        return (
          <Marker
            eventHandlers={{
              click: () => {
                dispatch(setActiveMarker(incident.incident_id));
              },
            }}
            position={[incident.lat, incident.long]}
            icon={L.divIcon({
              html: "<span>" + incident.incident_type + "</span>",
              className:
                "incident-marker flexbox " +
                LevelToClass(incident.level) +
                " " +
                isActive,
            })}
          ></Marker>
        );
      })}
    </>
  );
}
/*
 *   Icon({
                html: "",
                c
                iconUrl: "icon.svg",
                className:
                  "incident-marker " +
                  LevelToClass(incident.level) +
                  " " +
                  isActive,
              })

 */
