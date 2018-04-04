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
    this._engine.input(
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
    this._engine.update();
    this._engine.render((pixels, width, height) => {
      this._draw(pixels, width, height);
    });
    return (this._frameId = window.requestAnimationFrame(() => this._loop()));
  }

  _draw(pixels, width, height) {
    const canvas = document.getElementById("screen");
    if (canvas) {
      canvas.width = width * this._scale;
      canvas.height = height * this._scale;
      const context = canvas.getContext("2d");
      context.scale(this._scale, this._scale);
      for (let j = 0; j < height; j += 1) {
        for (let i = 0; i < width; i += 1) {
          const index = 4 * (j * width + i);
          const r = pixels[index + 0];
          const g = pixels[index + 1];
          const b = pixels[index + 2];
          const a = pixels[index + 3];
          context.fillStyle = `rgba(${r},${g},${b},${a / 100})`;
          context.fillRect(i, j, 1, 1);
        }
      }
    }
  }
}
