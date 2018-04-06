import { BrowserWindow, Menu, ipcMain } from "electron";

function sendToFocusedWindow(...args) {
  BrowserWindow.getFocusedWindow().webContents.send(...args);
}

const templates = [
  ...(process.platform === "darwin"
    ? [
        {
          label: "Emu",
          submenu: [{ role: "about" }, { type: "separator" }, { role: "quit" }]
        }
      ]
    : []),

  {
    label: "View",
    submenu: [
      {
        label: "Actual Size",
        accelerator: "CmdOrCtrl+0",
        click() {
          sendToFocusedWindow("accelerator", "ACTUAL_SIZE");
          console.log("0");
        }
      },
      {
        label: "Zoom In",
        accelerator: "CmdOrCtrl+Plus",
        click() {
          sendToFocusedWindow("accelerator", "ZOOM_IN");
          console.log("+");
        }
      },
      {
        label: "Zoom Out",
        accelerator: "CmdOrCtrl+-",
        click() {
          sendToFocusedWindow("accelerator", "ZOOM_OUT");
          console.log("-");
        }
      }
    ]
  }
];

export default () => Menu.setApplicationMenu(Menu.buildFromTemplate(templates));
