import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

type DirDetails = {
  path: string;
  name: string;
  size: number;
  metadata: string;
};

function App() {
  const [dirContent, setDirContent] = useState<string[]>([]);
  const [directory, setDirectory] = useState("");

  const [clickCount, setClickCount] = useState(0);
  const [lastClickTime, setLastClickTime] = useState(0);

  const [dirD, setDirD] = useState<DirDetails[]>([]);

  useEffect(() => {
    onCreate();
  }, []);

  {/* const onCreate = async () => { */}
  {/*   setDirContent(await invoke("home_dir")); */}
  {/* } */}

  const onCreate = async () => {
    setDirD(await invoke("home_dir"));
  }

  const openDir = async (dirname : string) => {
    setDirContent(await invoke("open_dir", { dirname }));
  }

  const goBack = async () => {
    setDirContent(await invoke("go_back", { directory }));
  }

  return (
    <div className="container">
      <button onClick={goBack}>Go Back</button>
      <h1>Wormhole</h1>
      {dirD.map((dir) => (
        <form
          className="row"
          onSubmit={(e) => {
            e.preventDefault();
            const now = new Date().getTime();
            const timeSinceLastClick = now - lastClickTime;
            if (timeSinceLastClick < 500) {
              openDir(directory);
            } else {
              setClickCount(clickCount + 1);
              setLastClickTime(now);
              setDirectory(dir.path);
            }
          }}
        >
          <button type="submit">{dir.path}</button>
        </form>
      ))}
    </div>
  );
}

export default App;
