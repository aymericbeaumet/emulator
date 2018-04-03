import React from "react";
import styled from "styled-components";

import Core from "core";

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
    this._core = new Core();
    this._width = 160;
    this._height = 144;
    this._scale = 2;
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
    }
  }

  componentWillUnmount() {
    window.cancelAnimationFrame(this._frameId);
    this._core.delete();
  }

  _loop() {
    this._core.input();
    this._core.update();
    this._core.render(() => {
      this._draw();
    });
    return (this._frameId = window.requestAnimationFrame(() => this._loop()));
  }

  _draw() {
    const canvas = document.getElementById("screen");
    if (canvas) {
      canvas.width = this._width * this._scale;
      canvas.height = this._height * this._scale;
      const context = canvas.getContext("2d");
      context.scale(this._scale, this._scale);
      for (let j = 0; j < this._height; j += 1) {
        for (let i = 0; i < this._width; i += 1) {
          if ((i + j) % 2) {
            context.fillStyle = "rgba(50,50,50,0.5)";
            context.fillRect(i, j, 1, 1);
          }
        }
      }
    }
  }
}
