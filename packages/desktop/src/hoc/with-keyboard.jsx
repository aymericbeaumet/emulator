import React from "react";
import getComponentName from "../utils/get-component-name";

class Keyboard {
  constructor() {
    this._onKeyUp = event => {
      this[event.keyCode] = 0;
    };

    this._onKeyDown = event => {
      this[event.keyCode] = 1;
    };
  }

  addEventListeners() {
    window.addEventListener("keyup", this._onKeyUp);
    window.addEventListener("keydown", this._onKeyDown);
  }

  removeEventListeners() {
    window.removeEventListener("keyup", this._onKeyUp);
    window.removeEventListener("keydown", this._onKeyDown);
  }
}

export default function withKeyboard(WrappedComponent) {
  const hoc = class WithKeyboard extends React.Component {
    constructor() {
      super();
      this._keyboard = new Keyboard();
    }

    componentDidMount() {
      this._keyboard.addEventListeners();
    }

    componentWillUnmount() {
      this._keyboard.removeEventListeners();
    }

    render() {
      return <WrappedComponent keyboard={this._keyboard} {...this.props} />;
    }
  };
  hoc.displayName = `${hoc.name}(${getComponentName(WrappedComponent)})`;
  return hoc;
}
