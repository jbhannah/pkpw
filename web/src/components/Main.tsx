import PasswordBox from "./PasswordBox";

const Main = () => (
  <main role="main" class="flex flex-col items-center mt-8 p-4">
    <h1 class="text-6xl mb-6 text-center">pkpw</h1>
    <h2 class="text-xl mb-6 text-center max-w-xl">
      What if{" "}
      <a
        class="link link-accent"
        href="https://xkcd.com/936/"
        target="_blank"
        rel="noreferrer noopener"
      >
        correct horse battery staple
      </a>
      {", "}but Pokémon.
    </h2>
    <div class="w-full max-w-lg mt-8 mb-16">
      <PasswordBox />
    </div>
    <p class="max-w-2xl text-center mb-4">
      A password generator that creates strong passwords from Pokémon names.
      Uses the{" "}
      <a
        class="link link-accent"
        href="https://github.com/jbhannah/pkpw"
        target="_blank"
        rel="noreferrer noopener"
      >
        pkpw
      </a>{" "}
      Rust library compiled to WASM to generate passwords entirely in the
      browser.
    </p>
    <p class="max-w-2xl text-center mb-4">
      <strong>No passwords are sent over the network,</strong> only a counter of
      the number of times the page is visited and each button is clicked.
    </p>
    <p class="max-w-2xl text-center mb-4">
      All Pokémon names are ™ and ©{" "}
      <a
        class="link link-accent"
        href="https://www.pokemon.com"
        target="_blank"
        rel="noreferrer noopener"
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
        rel="noreferrer noopener"
      >
        Jesse Brooklyn Hannah
      </a>
      {"."}
    </p>
  </main>
);

export default Main;
