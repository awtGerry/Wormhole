import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";
import addFolder from "./assets/icons/add-folder.png";
import addFile from "./assets/icons/add-file.png";
import folderImg from "./assets/icons/folder.svg";
import fileImg from "./assets/icons/file.svg";
import { Dropdown, Menu } from "antd";

type DirDetails = {
  path: string;
  name: string;
  size: number;
  metadata: string;
};

function App() {
  const [directory, setDirectory] = useState("");
  const [dirD, setDirD] = useState<DirDetails[]>([]);

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
          icon: <img src={addFolder} className="menu_logo" alt ="logo" />,
          onClick: () => { create_dir("wormdir") }
        },
        {
          label: "Create File",
          key: "2",
          icon: <img src={addFile} className="menu_logo" alt ="logo" />,
          onClick: () => { create_file("wormfile.txt") }
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

  const check_dir = (dir : DirDetails) => {
    if (dir.metadata === "dir") {
      return true;
    } else {
      return false;
    }
  }

  return (
    <div>
      <Dropdown overlay={menu} trigger="contextMenu">
        <div>
          <h1>Wormhole</h1>
          <button onClick={goBack}></button>
          <div className="container">
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
                    setLastClickTime(now);
                    setDirectory(dir.path);
                  }
                }}
              >
                <div className="folder">
                  <button type="submit">
                    {check_dir(dir) ? (
                      <img src={folderImg} className="folder-img" alt ="logo" />
                    ) : (
                      <img src={fileImg} className="folder-img" alt ="logo" />
                    )}
                    {/* <img src={folderImg} className="folder-img" alt ="logo" /> */}
                    <p className="folder-name">{dir.name}</p>
                  </button>
                </div>
              </form>
            ))}
          </div>
        </div>
      </Dropdown>
    </div>
  );
}

export default App;
