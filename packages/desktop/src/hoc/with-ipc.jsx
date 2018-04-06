import { ipcRenderer } from "electron";
import React from "react";
import getComponentName from "../utils/get-component-name";

export default function withIPC(WrappedComponent) {
  const hoc = class WithIPC extends React.Component {
    render() {
      return <WrappedComponent ipc={ipcRenderer} {...this.props} />;
    }
  };
  hoc.displayName = `${hoc.name}(${getComponentName(WrappedComponent)})`;
  return hoc;
}
