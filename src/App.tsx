import { useEffect, useState } from "react";

import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";
import { Input } from "./components/ui/input";
import { Card } from "./components/ui/card";
import { Button } from "./components/ui/button";

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
    <main
      className="container dark h-screen min-w-full flex flex-col items-center justify-center "
      style={{ backgroundColor: "var(--background)" }}
    >
      <h1 className="text-2xl text-foreground font-bold">{`Welcome to RomM-en`}</h1>
      <div className="flex flex-col gap-2 items-center justify-center h-full w-full">
        <Card className="flex flex-col gap-2 w-full max-w-md p-4">
          <label htmlFor="serverUrl">Server URL</label>
          <Input
            type="text"
            value={serverUrl}
            onChange={(e) => setServerUrl(e.target.value)}
          />

          <Button onClick={handleAuthenticate}>Authenticate</Button>
          {isAuthenticated && <p>Authenticated!</p>}
        </Card>
      </div>
    </main>
  );
}

export default App;
