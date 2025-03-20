import { render } from "preact";

import { PostHogProvider } from "posthog-js/react";
import { Footer } from "./components/Footer";
import { Header } from "./components/Header";
import { PasswordBox } from "./components/PasswordBox";
import "./style.css";

const options = {
  api_host: "https://us.i.posthog.com",
};

const REACT_APP_PUBLIC_POSTHOG_KEY =
  "phc_6rKhMfbzpccKI3EcDXbng8EuP7h2FC2rQga9nRBV8G";

const App = () => (
  <PostHogProvider apiKey={REACT_APP_PUBLIC_POSTHOG_KEY} options={options}>
    <div class="min-h-screen w-full flex flex-col">
      <Header />
      <main class="flex-1">
        <div class="flex flex-col items-center mt-8 p-4">
          <h1 class="text-6xl mb-6 text-center">pkpw</h1>
          <h2 class="text-xl mb-6 text-center max-w-xl">
            What if{" "}
            <a
              class="link link-accent"
              href="https://xkcd.com/936/"
              target="_blank"
              rel="noopener"
            >
              correct horse battery staple
            </a>
            {", "}but Pokémon.
          </h2>
          <div class="w-full max-w-lg mt-8 mb-16">
            <PasswordBox />
          </div>
          <p class="max-w-2xl text-center mb-4">
            A password generator that uses Pokémon names to generate strong
            passwords.{" "}
            <a
              class="link link-accent"
              href="https://github.com/jbhannah/pkpw"
              target="_blank"
              rel="noopener"
            >
              Learn more
            </a>
          </p>
          <p class="max-w-2xl text-center mb-4">
            All Pokémon names are ™ and ©{" "}
            <a
              class="link link-accent"
              href="https://www.pokemon.com"
              target="_blank"
              rel="noopener"
            >
              The Pokémon Company
            </a>
            {"."}
          </p>
          <p class="max-w-2xl text-center">
            Created by{" "}
            <a
              class="link link-accent"
              href="https://jbhannah.net"
              target="_blank"
              rel="noopener"
            >
              Jesse Brooklyn Hannah
            </a>
            {"."}
          </p>
        </div>
      </main>
      <Footer />
    </div>
  </PostHogProvider>
);

const parent = document.getElementById("app");
parent && render(<App />, parent);
