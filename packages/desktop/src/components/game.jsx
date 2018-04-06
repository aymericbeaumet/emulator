import React from "react";
import styled from "styled-components";

import core from "core";

const Style = styled.div`
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;

  canvas {
    border: 30px solid black;
    border-radius: 20px;
  }
`;

export default class Game extends React.Component {
  constructor() {
    super();
    this._engine = new core.game.engine.Engine();
    this._scale = 2;
    // watch keys
    this._keys = {};
    this._onKeyUp = event => {
      this._keys[event.keyCode] = 0;
    };
    this._onKeyDown = event => {
      this._keys[event.keyCode] = 1;
    };
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
      this._frameId = this._loop();
      window.addEventListener("keyup", this._onKeyUp);
      window.addEventListener("keydown", this._onKeyDown);
    }
  }

  componentWillUnmount() {
    window.cancelAnimationFrame(this._frameId);
    window.removeEventListener("keyup", this._onKeyUp);
    window.removeEventListener("keydown", this._onKeyDown);
    this._engine.delete();
  }

  _loop() {
    this._engine.input(this._getKeyboard());
    this._engine.render((pixels, width, height) => {
      this._draw(pixels, width, height);
    });
    return (this._frameId = window.requestAnimationFrame(() => this._loop()));
  }

  _getKeyboard() {
    return (
      (this._keys[16] << 0) | // select (shift)
      (this._keys[32] << 1) | // start (spacebar)
      (this._keys[37] << 2) | // left
      (this._keys[38] << 3) | // up
      (this._keys[39] << 4) | // right
      (this._keys[40] << 5) | // down
      (this._keys[65] << 6) | // a
      (this._keys[66] << 7) | // b
      0
    );
  }

  _draw(pixels, width, height) {
    const canvas = document.getElementById("screen");
    if (canvas) {
      if (
        !(
          this._lastScale === this.scale &&
          this._lastWidth === width &&
          this._lastHeight === height
        )
      ) {
        canvas.width = width;
        canvas.height = height;
        canvas.style.width = `${width * this._scale}px`;
        canvas.style.height = `${height * this._scale}px`;
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
