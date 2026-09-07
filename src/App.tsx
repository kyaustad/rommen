import { useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [serverUrl, setServerUrl] = useState("");

  async function handleAuthenticate() {
    await invoke("authenticate", {
      serverUrl,
    });
  }

  return (
    <main className="container">
      <h1>Welcome to Rommen</h1>
      <div className="flex flex-col gap-2">
        <label htmlFor="serverUrl">Server URL</label>
        <input
          type="text"
          value={serverUrl}
          onChange={(e) => setServerUrl(e.target.value)}
        />

        <button onClick={handleAuthenticate}>Authenticate</button>
      </div>
    </main>
  );
}

export default App;
