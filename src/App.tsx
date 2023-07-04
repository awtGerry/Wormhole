import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { Dropdown, Menu } from "antd";
import "./App.css";

type DirDetails = {
  path: string;
  name: string;
  size: number;
  metadata: string;
};

function App() {
  const [directory, setDirectory] = useState("");
  const [dirD, setDirD] = useState<DirDetails[]>([]);

  const [clickCount, setClickCount] = useState(0);
  const [lastClickTime, setLastClickTime] = useState(0);

  const menu = (
    <Menu
      style={{
        minWidth: 200,
        minHeight: 200,
        padding: 10,
      }}
      items={[
        {
          label: "Create Directory",
          key: "1",
          onClick: () => { create_dir("wormdir") }
        },
        {
          label: "Create File",
          key: "2",
          onClick: () => { create_file("wormfile") }
        }
      ]}
    ></Menu>
  );

  useEffect(() => {
    onCreate();
  }, []);

  const onCreate = async () => {
    setDirD(await invoke("home_dir"));
  }

  const openDir = async (dirname : string) => {
    setDirD(await invoke("open_dir", { dirname }));
  }

  const goBack = async () => {
    setDirD(await invoke("go_back", { directory }));
  }

  const create_dir = async (dirname : string) => {
    await invoke("create_dir", { dirname });
  }

  const create_file = async (dirname : string) => {
    await invoke("create_file", { dirname });
  }

  return (
    <div>
      <Dropdown overlay={menu} trigger="contextMenu">
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
              <button type="submit">{dir.name}</button>
            </form>
          ))}
        </div>
      </Dropdown>
    </div>
  );
}

export default App;
