import { defineConfig } from "@rsbuild/core";
import { pluginPreact } from "@rsbuild/plugin-preact";

export default defineConfig({
  plugins: [pluginPreact()],
  html: {
    template: "./index.html",
  },
  dev: {
    hmr: false,
  },
});
