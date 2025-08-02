import { test, expect } from "@playwright/test";

test("Angular default app shows some text", async ({ page }) => {
    await page.goto("/home");
    await expect(page.locator("body")).not.toBeEmpty();
});
