import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [dirs, setDirs] = useState<string[]>([]);
  const [dirToOpen, setDirToOpen] = useState<string>("");

  async function dir() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setDirs(await invoke("home_dir"));
  }

  const openDir = (dirname: string) => {
    invoke("open_dir", { dirname })
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
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            openDir(dir);
          }}
        >
          <button type="submit">{dir}</button>
        </form>
        /* <form */
        /*   className="row" */
        /*   onSubmit={(e) => { */
        /*     e.preventDefault; */
        /*     {/1* openDir(dir); *1/} */
        /*   }} */
        /* > */
        /*   <button type="submit">{dir}</button> */
        /* </form> */
        // <button>{dir}</button>
      ))}
    </div>
  );
}

export default App;
