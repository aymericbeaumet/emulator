import React from "react";
import styled from "styled-components";

import Game from "./Game";
import withKeyboard from "../hoc/with-keyboard";

const GameWithKeyboard = withKeyboard(Game);

const Style = styled.div`
  width: 100%;
  height: 100%;
`;

export default class App extends React.Component {
  render() {
    return (
      <Style>
        <GameWithKeyboard />
      </Style>
    );
  }
}
