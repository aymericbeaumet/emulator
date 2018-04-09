import { BrowserWindow } from "electron";

export default function dispatchToFocusedWindow(...args) {
  return BrowserWindow.getFocusedWindow().webContents.send("action", ...args);
}
