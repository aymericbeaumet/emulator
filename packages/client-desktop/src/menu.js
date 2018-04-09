import { BrowserWindow, Menu, dialog } from "electron";
import { productName } from "../package.json";
import isDevMode from "./utils/isDevMode";
import dispatchToFocusedWindow from "./utils/dispatchToFocusedWindow";

const templates = [
  ...(process.platform === "darwin"
    ? [
        {
          label: productName,
          submenu: [{ role: "about" }, { type: "separator" }, { role: "quit" }]
        }
      ]
    : []),

  {
    label: "File",
    submenu: [
      {
        label: "Open…",
        accelerator: "CmdOrCtrl+o",
        click() {
          dialog.showOpenDialog(
            {
              properties: ["openFile"],
              filters: [{ name: "Game Boy ROMs", extensions: ["gb", "gbc"] }]
            },
            filePaths => {
              dispatchToFocusedWindow({ type: "OPEN_FILES", filePaths });
            }
          );
        }
      }
    ]
  },

  {
    label: "View",
    submenu: [
      {
        label: "Actual Size",
        accelerator: "CmdOrCtrl+0",
        click() {
          dispatchToFocusedWindow({ type: "ACTUAL_SIZE" });
        }
      },
      {
        label: "Zoom In",
        accelerator: "CmdOrCtrl+Plus",
        click() {
          dispatchToFocusedWindow({ type: "ZOOM_IN" });
        }
      },
      {
        label: "Zoom Out",
        accelerator: "CmdOrCtrl+-",
        click() {
          dispatchToFocusedWindow({ type: "ZOOM_OUT" });
        }
      }
    ]
  },

  ...(isDevMode
    ? [
        {
          label: "Develop",
          submenu: [
            {
              label: "Reload",
              accelerator: "Cmd+R",
              click() {
                BrowserWindow.getFocusedWindow().webContents.reload();
              }
            },
            {
              label: "Reload without cache",
              accelerator: "Cmd+Shift+R",
              click() {
                BrowserWindow.getFocusedWindow().webContents.reloadIgnoringCache();
              }
            },
            { type: "separator" },
            {
              label: "Toggle Web Inspector",
              accelerator: "CmdOrCtrl+Alt+I",
              click() {
                BrowserWindow.getFocusedWindow().webContents.toggleDevTools();
              }
            },
            {
              label: "Toggle Performance Overlay",
              accelerator: "CmdOrCtrl+Alt+P",
              click() {
                dispatchToFocusedWindow({ type: "TOGGLE_PERFORMANCE_OVERLAY" });
              }
            }
          ]
        }
      ]
    : [])
];

export default () => Menu.setApplicationMenu(Menu.buildFromTemplate(templates));
