import { render } from "preact";

import { Password } from "./components/password";
import "./style.css";

export function App() {
  return (
    <div class="min-h-screen w-full flex flex-col">
      <div>Header</div>
      <div class="flex-1 flex flex-col justify-center p-4">
        <Password />
      </div>
      <div>Footer</div>
    </div>
  );
}

const parent = document.getElementById("app");
parent && render(<App />, parent);
