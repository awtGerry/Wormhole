import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [dirContent, setDirContent] = useState<string[]>([]);
  const [directory, setDirectory] = useState("");

  const [clickCount, setClickCount] = useState(0);
  const [lastClickTime, setLastClickTime] = useState(0);

  useEffect(() => {
    onCreate();
  }, []);

  const onCreate = async () => {
    setDirContent(await invoke("home_dir"));
  }

  const openDir = async (dirname : string) => {
    setDirContent(await invoke("open_dir", { dirname }));
  }

  return (
    <div className="container">
      <h1>Wormhole</h1>
      {dirContent.map((dir) => (
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            const now = new Date().getTime();
            const timeSinceLastClick = now - lastClickTime;
            if (timeSinceLastClick < 300) {
              openDir(directory);
            } else {
              setClickCount(clickCount + 1);
              setLastClickTime(now);
              setDirectory(dir);
            }
          }}
        >
          <button type="submit">{dir}</button>
        </form>
      ))}
    </div>
  );
}

export default App;
