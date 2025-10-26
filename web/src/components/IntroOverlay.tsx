import { type SetStateAction, type Dispatch } from "react";

interface IntroOverlayProps {
  killer: Dispatch<SetStateAction<boolean>>;
}

export default function IntroOverlay(props: IntroOverlayProps) {
  return (
    <div className="overlay-full">
      <img
        style={{
          height: "3rem",
          filter:
            "invert(100%) sepia(1%) saturate(4409%) hue-rotate(192deg) brightness(116%) contrast(100%)",
        }}
        src="logo.svg"
      />
      <h2 style={{ fontWeight: 500, paddingTop: ".5em", fontSize: "1.65em" }}>
        fear thy neighbor.
      </h2>
      <h3 style={{ fontWeight: 300, fontSize: "1.35em" }}>
        pay us $6/week or you're not safe.
      </h3>
      <button
        onClick={() => {
          props.killer(false);
        }}
      >
        Start 36 Hour Free Trial →
      </button>
      <div
        className="flexbox"
        style={{
          gap: "1.5em",
          textDecoration: "underline",
          marginTop: "2em",
          fontWeight: 200,
          fontSize: "1.2em",
          fontFamily: "Lato",
        }}
      >
        <h5>
          <a href="/about">About Us</a>
        </h5>
        <h5>
          <a href="/privacy">Privacy Policy / Legal</a>
        </h5>
        <h5>
          <a href="mailto:press@lying.club">Contact</a>
        </h5>
      </div>
    </div>
  );
}
