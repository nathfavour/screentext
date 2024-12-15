import "./App.css";

import React from "react";

  

function App() {
  return (
    <main
      data-tauri-drag-region
      className="container"
      style={{ WebkitAppRegion: "drag", width: "100vw", height: "100vh" } as React.CSSProperties}
    >
      {/* Blank content */}
    </main>
  );
}

export default App;
