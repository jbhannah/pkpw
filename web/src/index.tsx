import { render } from "preact";
import App from "./App";

const parent = document.getElementById("app");
parent && render(<App />, parent);
