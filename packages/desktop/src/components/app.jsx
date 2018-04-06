import React from "react";
import styled from "styled-components";

import Emulator from "./emulator";
import withIPC from "../hoc/with-ipc";
import withKeyboard from "../hoc/with-keyboard";

const EnhancedEmulator = withIPC(withKeyboard(Emulator));

const Style = styled.div`
  width: 100%;
  height: 100%;
`;

export default class App extends React.Component {
  render() {
    return (
      <Style>
        <EnhancedEmulator />
      </Style>
    );
  }
}
