import { ThemeToggle } from "./ThemeToggle";

export const Header = () => (
  <header class="justify-between p-4 flex flex-row w-full">
    <h1 class="text-2xl">
      <a class="link link-accent link-hover" href="https://pkpw.jbhannah.net/">
        pkpw
      </a>
    </h1>
    <div>
      <ThemeToggle />
    </div>
  </header>
);
