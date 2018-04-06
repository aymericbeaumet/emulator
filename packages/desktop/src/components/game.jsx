import React from "react";
import styled from "styled-components";

import core from "core";

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

export default class Game extends React.Component {
  constructor({ keyboard }) {
    super();
    this._emulator = new core.emulators.gameboycolor.GameBoyColor();
    this._keyboard = keyboard;
    this._scale = 2;
  }

  shouldComponentUpdate() {
    return false;
  }

  render() {
    return (
      <Style>
        <canvas id="screen" />
      </Style>
    );
  }

  componentDidMount() {
    if (!this._frameId) {
      this._loop();
    }
  }

  componentWillUnmount() {
    window.cancelAnimationFrame(this._frameId);
    this._emulator.delete();
  }

  _loop() {
    if (window.__stats__) {
      window.__stats__.begin();
    }
    this._emulator.input(this._getKeyboard());
    this._emulator.render((pixels, width, height) => {
      this._draw(pixels, width, height);
    });
    if (window.__stats__) {
      window.__stats__.end();
    }
    this._frameId = window.requestAnimationFrame(() => this._loop());
  }

  _getKeyboard() {
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
}
