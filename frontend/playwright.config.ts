import { defineConfig, devices } from "@playwright/test";
export default defineConfig({
  timeout:150_000,
  testDir:"./tests",fullyParallel:false,retries:0,reporter:[["json",{outputFile:"../.evidence/playwright-report.json"}],["line"]],
  use:{baseURL:"http://127.0.0.1:4173",trace:"on",screenshot:"only-on-failure"},
  projects:[{name:"chromium",use:{...devices["Desktop Chrome"]}}]
});
