export interface Incident {
  incident_id: string;
  title: string;
  reporter: string;
  time_unix: number;
  lat: number;
  long: number;
  address: string;
  incident_type: string;
  child_of: string | null; // incident_id
  description: string;
  level: number;
}
