import { Incidents } from "./map/Incidents";
import { MapContainer, TileLayer } from "react-leaflet";
import { CurrentLocation } from "./map/CurrentLocation";

export default function Map() {
  return (
    <MapContainer
      center={[34.05112818362247, -118.25362376905395]}
      zoom={15}
      className="map-container"
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://api.mapbox.com/styles/v1/mapbox/dark-v11/tiles/256/{z}/{x}/{y}@2x?access_token=pk.eyJ1IjoicXBhdHdhcmRoYW4tb3h5IiwiYSI6ImNtMG9hdTdvZjA2bW0ybW9pOG9lMnl5ZTEifQ.qIkMeJEiiQTMx5TZM0bxOg"
      />
      <CurrentLocation />
      <Incidents />
    </MapContainer>
  );
}
