import { type Incident } from "../structs/Incident";

export async function GetIncidents(): Promise<Array<Incident>> {
  const response = await fetch("http://localhost:1947/incidents");
  if (!response.ok) {
    console.log(response.statusText);
    return [];
  }
  let out: Array<Incident> = await response.json();
  console.log(out);
  return out;
}
