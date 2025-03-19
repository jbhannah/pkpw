import { render } from "preact";

import { Password } from "./components/password";
import { ThemeToggle } from "./components/theme-toggle";
import "./style.css";

export function App() {
  return (
    <div class="min-h-screen w-full flex flex-col">
      <header class="justify-between p-4 flex flex-row w-full">
        <h1 class="text-2xl">
          <a
            class="link link-accent link-hover"
            href="https://pkpw.jbhannah.net/"
          >
            pkpw
          </a>
        </h1>
        <div>
          <ThemeToggle />
        </div>
      </header>
      <div class="flex-1 flex flex-col items-center mt-8 p-4">
        <h1 class="text-6xl mb-6 text-center">pkpw</h1>
        <h2 class="text-xl mb-6 text-center max-w-xl">
          What if{" "}
          <a class="link link-accent" href="https://xkcd.com/936/">
            correct horse battery staple
          </a>
          {", "}but Pokémon.
        </h2>
        <div class="w-full max-w-lg mt-8 mb-16">
          <Password />
        </div>
        <p class="max-w-2xl text-center">
          A password generator that uses Pokémon names to generate strong
          passwords.{" "}
          <a class="link link-accent" href="https://github.com/jbhannah/pkpw">
            Learn more
          </a>
        </p>
      </div>
      <footer class="p-4 flex flex-row w-full justify-center gap-x-2">
        <a class="link link-accent" href="https://github.com/jbhannah/pkpw">
          GitHub
        </a>
        <a class="link link-accent" href="https://crates.io/crates/pkpw">
          crates.io
        </a>
        <a class="link link-accent" href="https://npmjs.com/package/pkpw">
          npm
        </a>
      </footer>
    </div>
  );
}

const parent = document.getElementById("app");
parent && render(<App />, parent);
