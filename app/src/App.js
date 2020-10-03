import React from "react";
import { Provider } from "urql";

import "./App.css";
import Humans from "./components/Humans";
import { client } from "./graphql";

function App() {
  return (
    <Provider value={client}>
      <div className="App">
        <header className="App-header">
          <p>
            Edit <code>src/App.js</code> and save to reload.
          </p>
          <Humans />
          <a
            className="App-link"
            href="/graphql"
            target="_blank"
            rel="noopener noreferrer"
          >
            Open Playground
          </a>
        </header>
      </div>
    </Provider>
  );
}

export default App;
