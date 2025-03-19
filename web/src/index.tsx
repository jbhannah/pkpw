import { render } from "preact";

import { Password } from "./components/password";
import { ThemeToggle } from "./components/theme-toggle";
import "./style.css";

export function App() {
  return (
    <div class="min-h-screen w-full flex flex-col">
      <header>
        <ThemeToggle />
      </header>
      <div class="flex-1 flex flex-col items-center justify-center p-4">
        <h1>pkpw</h1>
        <p>What if correct horse battery staple, but Pokémon.</p>
        <Password />
      </div>
      <footer>Footer</footer>
    </div>
  );
}

const parent = document.getElementById("app");
parent && render(<App />, parent);
