import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

function App() {
  const [serverUrl, setServerUrl] = useState("");
  const [isAuthenticated, setIsAuthenticated] = useState(false);

  async function handleAuthenticate() {
    await invoke("authenticate", {
      serverUrl,
    });
  }

  useEffect(() => {
    listen("auth_success", () => {
      console.log("Authentication successful");
      setIsAuthenticated(true);
    });
  }, []);

  return (
    <main className="container">
      <h1>{`Welcome to RomM-en`}</h1>
      <div className="flex flex-col gap-2">
        <label htmlFor="serverUrl">Server URL</label>
        <input
          type="text"
          value={serverUrl}
          onChange={(e) => setServerUrl(e.target.value)}
        />

        <button onClick={handleAuthenticate}>Authenticate</button>
        {isAuthenticated && <p>Authenticated!</p>}
      </div>
    </main>
  );
}

export default App;
