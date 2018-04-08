import { app, BrowserWindow } from "electron";
import installExtension, {
  REACT_DEVELOPER_TOOLS
} from "electron-devtools-installer";
import { enableLiveReload } from "electron-compile";
import menu from "./menu";
import isDevMode from "./utils/isDevMode";

let mainWindow;

if (isDevMode) {
  enableLiveReload({ strategy: "react-hmr" });
}

const createWindow = async () => {
  mainWindow = new BrowserWindow();
  mainWindow.maximize();
  mainWindow.loadURL(`file://${__dirname}/index.html`);
  if (isDevMode) {
    await installExtension(REACT_DEVELOPER_TOOLS);
    mainWindow.webContents.openDevTools();
  }
  mainWindow.on("closed", () => {
    mainWindow = null;
  });
};

app.on("ready", () => {
  createWindow();
  menu();
});

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (mainWindow === null) {
    createWindow();
  }
});
