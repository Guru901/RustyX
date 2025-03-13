import { defineConfig } from "@playwright/test";

export default defineConfig({
  use: {
    baseURL: "http://127.0.0.1:3000", // Change this if needed
    extraHTTPHeaders: {
      "User-Agent": "Playwright-Test",
    },
  },
});
