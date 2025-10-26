import React from "react";
import Map from "../components/Map";
import ReportOverlay from "../components/ReportOverlay";
import IntroOverlay from "../components/IntroOverlay";

export default function Home() {
  const [showIntroOverlay, setShowIntroOverlay] = React.useState<boolean>(true);

  return (
    <>
      {!showIntroOverlay ? <img src="logo.svg" className="logo" /> : ""}
      <div className="report-overlay"></div>
      <button className="report">
        📸 Sensationalize <br /> Something
      </button>
      {}
      {showIntroOverlay ? <IntroOverlay killer={setShowIntroOverlay} /> : ""}
      <Map />
      <ReportOverlay />
    </>
  );
}
