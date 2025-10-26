import { useMap } from "react-leaflet";

export function CurrentLocation() {
  const map = useMap();

  navigator.geolocation.getCurrentPosition((position) => {
    map.setView([position.coords.latitude, position.coords.longitude], 14);
  });

  console.log("map center:", map.getCenter());
  return null;
}
