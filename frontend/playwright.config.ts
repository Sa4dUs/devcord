import { defineConfig, devices } from "@playwright/test";

export default defineConfig({
    testDir: "./tests", // folder with your tests
    timeout: 30 * 1000, // 30 seconds timeout per test
    retries: 0, // no retries on failure
    use: {
        baseURL: "http://localhost:4200", // Angular app URL
        headless: true, // run browser in headless mode
        viewport: { width: 1280, height: 720 },
        video: "retain-on-failure", // record video only if test fails
        screenshot: "only-on-failure", // take screenshot only on failure
    },
    projects: [
        { name: "chromium", use: { ...devices["Desktop Chrome"] } },
        { name: "firefox", use: { ...devices["Desktop Firefox"] } },
        { name: "webkit", use: { ...devices["Desktop Safari"] } },
    ],
    // This section starts your Angular server automatically before tests
    webServer: {
        command: "ng serve --port 4200",
        port: 4200,
        reuseExistingServer: true, // if server is already running, reuse it
        timeout: 120 * 1000, // wait up to 2 minutes for the server to start
        stdout: "pipe", // optional: pipe server logs (can remove if you want)
        stderr: "pipe",
    },
});
