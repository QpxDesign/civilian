export default function About() {
  return (
    <>
      <div
        className="flexbox"
        style={{ paddingTop: "2em", alignItems: "center", gap: "1.5em" }}
      >
        <a href="/">
          <img
            src="https://upload.wikimedia.org/wikipedia/commons/d/d9/Noun_Project_down_arrow_icon_719904_cc.svg"
            style={{
              filter:
                "invert(100%) sepia(1%) saturate(4409%) hue-rotate(192deg) brightness(116%) contrast(100%)",
              transform: "rotate(90deg)",
              height: "3rem",
            }}
          />
        </a>
        <a href="/">
          <img
            src="logo.svg"
            style={{
              height: "2.5rem",
              filter:
                "invert(100%) sepia(1%) saturate(4409%) hue-rotate(192deg) brightness(116%) contrast(100%)",
            }}
          />
        </a>
      </div>
    </>
  );
}
