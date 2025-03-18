import { generate_password as pkpw } from "pkpw";
import { render } from "preact";

import "./style.css";

export function App() {
  return <div>{pkpw()}</div>;
}

const parent = document.getElementById("app");
parent && render(<App />, parent);
