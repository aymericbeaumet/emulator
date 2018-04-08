import React from "react";
import styled from "styled-components";
import _ from "lodash";
import { webFrame } from "electron";
import Stats from "stats.js";
import Helmet from "react-helmet";
import core from "core";
import withIPC from "../hoc/with-ipc";
import withKeyboard from "../hoc/with-keyboard";
import eventPreventDefault from "../utils/event-prevent-default";
import isDevMode from "../../utils/isDevMode";

const Style = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background-color: #272727;

  canvas {
    border: 30px solid black;
    border-radius: 20px;
  }
`;

const DEFAULT_SCALE = 2;
const MIN_SCALE = 1;
const MAX_SCALE = 4;
const SCALE_STEP = 0.5;

class App extends React.Component {
  constructor({ ipc, keyboard, scale = DEFAULT_SCALE }) {
    super();
    this._ipc = ipc;
    this._keyboard = keyboard;
    this._scale = scale;

    // Disable zoom
    webFrame.setVisualZoomLevelLimits(1, 1);
    webFrame.setLayoutZoomLevelLimits(0, 0);

    if (isDevMode) {
      this._handleAction({ type: "TOGGLE_PERFORMANCE_OVERLAY" });
    }
  }

  componentDidMount() {
    this._emulator = new core.emulators.gameboycolor.GameBoyColor();
    this._loop();
    this._ipc.on("action", (channel, action) => this._handleAction(action));
    window.document.addEventListener("dragover", eventPreventDefault);
    window.document.addEventListener("drop", eventPreventDefault);
    window.document.body.addEventListener("drop", this._onDrop);
  }

  shouldComponentUpdate() {
    return false;
  }

  _getInputs() {
    return (
      (this._keyboard[16] << 0) | // select (shift)
      (this._keyboard[32] << 1) | // start (spacebar)
      (this._keyboard[37] << 2) | // left
      (this._keyboard[38] << 3) | // up
      (this._keyboard[39] << 4) | // right
      (this._keyboard[40] << 5) | // down
      (this._keyboard[65] << 6) | // a
      (this._keyboard[66] << 7) | // b
      0
    );
  }

  _draw(pixels, width, height) {
    const canvas = document.getElementById("screen");
    if (canvas) {
      if (
        this._lastScale !== this.scale ||
        this._lastWidth !== width ||
        this._lastHeight !== height
      ) {
        canvas.width = width;
        canvas.style.width = `${width * this._scale}px`;
        canvas.style.minWidth = canvas.style.width;
        canvas.style.maxWidth = canvas.style.width;
        canvas.height = height;
        canvas.style.height = `${height * this._scale}px`;
        canvas.style.minHeight = canvas.style.height;
        canvas.style.maxHeight = canvas.style.height;
        this._lastScale = this._scale;
        this._lastWidth = width;
        this._lastHeight = height;
      }
      this._imageData = this._imageData || new ImageData(width, height);
      this._imageData.data.set(pixels);
      canvas.getContext("2d").putImageData(this._imageData, 0, 0);
    }
  }

  _handleAction = ({ type, ...payload }) => {
    switch (type) {
      case "OPEN_FILES": {
        const [filePath] = payload.filePaths || [];
        if (filePath) {
          this._emulator.boot_with_file_path(filePath);
        }
        break;
      }
      case "ACTUAL_SIZE":
        this._scale = DEFAULT_SCALE;
        break;
      case "ZOOM_IN":
        this._scale = _.min([MAX_SCALE, this._scale + SCALE_STEP]);
        break;
      case "ZOOM_OUT":
        this._scale = _.max([MIN_SCALE, this._scale - SCALE_STEP]);
        break;
      case "TOGGLE_PERFORMANCE_OVERLAY":
        if (!this._stats) {
          this._stats = new Stats();
          this._stats.dom.id = "__stats__";
          document.body.appendChild(this._stats.dom);
        } else {
          document.body.removeChild(this._stats.dom);
          this._stats = null;
        }
        break;
      default:
        console.warn("Unhandled action", type); // eslint-disable-line no-console
    }
  };

  _onDrop = event => {
    event.preventDefault();
    this._handleAction({
      type: "OPEN_FILES",
      filePaths: Array.from(event.dataTransfer.files).map(file => file.path)
    });
    return false;
  };

  _loop = () => {
    if (this._stats) {
      this._stats.begin();
    }
    this._emulator.input(this._getInputs());
    this._emulator.render((pixels, width, height) => {
      this._draw(pixels, width, height);
    });
    if (this._stats) {
      this._stats.end();
    }
    this._frameId = window.requestAnimationFrame(this._loop);
  };

  render() {
    return (
      <React.Fragment>
        <Helmet>
          <style type="text/css">
            {`
            #__stats__ {
              pointer-events: none;
            }
            #__stats__ > canvas {
              float: left;
              display: block !important;
            }
            `}
          </style>
        </Helmet>
        <Style>
          <canvas id="screen" />
        </Style>
      </React.Fragment>
    );
  }
}

export default withIPC(withKeyboard(App));
