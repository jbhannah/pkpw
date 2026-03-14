import { PostHogProvider } from "posthog-js/react";
import Footer from "./components/Footer";
import Header from "./components/Header";
import Main from "./components/Main";

import "./assets/style.css";

const options = {
  api_host: "https://us.i.posthog.com",
};

const POSTHOG_KEY = import.meta.env.PUBLIC_POSTHOG_KEY;

const App = () => (
  <PostHogProvider apiKey={POSTHOG_KEY} options={options}>
    <div class="min-h-screen w-full flex flex-col">
      <Header />
      <div class="flex-1">
        <Main />
      </div>
      <Footer />
    </div>
  </PostHogProvider>
);

export default App;
