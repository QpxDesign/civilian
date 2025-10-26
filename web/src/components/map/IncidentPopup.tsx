import { type Incident } from "../../structs/Incident";
import { Popup } from "react-leaflet";

interface IncidentPopupProps {
  incident: Incident;
}

export function IncidentPopup(props: IncidentPopupProps) {
  return (
    <Popup>
      <h1>{props.incident.title}</h1>
    </Popup>
  );
}
