import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [dirs, setDirs] = useState<string[]>([]);

  async function dir() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setDirs(await invoke("home_dir"));
    // setDirs(await invoke("home_dir"));
  }

  return (
    <div className="container">
      <h1>Wormhole</h1>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          dir();
        }}
      >
        <button type="submit">Show home dir</button>
      </form>


      {dirs.map((dir) => (
        <a>{dir}</a>
      ))}
    </div>
  );
}

export default App;
