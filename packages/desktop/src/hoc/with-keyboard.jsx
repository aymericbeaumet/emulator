import React from "react";
import getComponentName from "../utils/get-component-name";

export default function withKeyboard(WrappedComponent) {
  const hoc = class WithKeyboard extends React.Component {
    constructor() {
      super();
      this._keyboard = {};
      this._onKeyUp = event => {
        this._keyboard[event.keyCode] = 0;
      };
      this._onKeyDown = event => {
        this._keyboard[event.keyCode] = 1;
      };
    }

    componentDidMount() {
      window.addEventListener("keyup", this._onKeyUp);
      window.addEventListener("keydown", this._onKeyDown);
    }

    componentWillUnmount() {
      window.removeEventListener("keyup", this._onKeyUp);
      window.removeEventListener("keydown", this._onKeyDown);
    }

    render() {
      return <WrappedComponent keyboard={this._keyboard} {...this.props} />;
    }
  };
  hoc.displayName = `${hoc.name}(${getComponentName(WrappedComponent)})`;
  return hoc;
}
