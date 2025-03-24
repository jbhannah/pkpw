import PasswordBox from "./PasswordBox";

const Main = () => (
  <main class="flex flex-col items-center mt-8 p-4">
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
      A password generator that uses Pokémon names to generate strong passwords.{" "}
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
  </main>
);

export default Main;
