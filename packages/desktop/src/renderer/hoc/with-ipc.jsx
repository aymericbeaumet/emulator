import { ipcRenderer } from "electron";
import React from "react";
import getComponentName from "../utils/get-component-name";

export default function withIPC(WrappedComponent) {
  const hoc = function WithIPC(props = {}) {
    return <WrappedComponent ipc={ipcRenderer} {...props} />;
  };
  hoc.displayName = `${hoc.name}(${getComponentName(WrappedComponent)})`;
  return hoc;
}
