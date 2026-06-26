import { test, expect } from './fixtures';
import type { Page } from '@playwright/test';

// seed.sql contains exactly 2 published questions; exhausting both triggers pool exhaustion.

async function exhaustPool(page: Page) {
  await page.goto('/');
  await page.getByTestId('option-a').click();
  await page.getByTestId('next-button').click();
  await page.getByTestId('option-a').click();
  await page.getByTestId('next-button').click();
}

test('after seeing all questions the pool exhaustion screen is shown', async ({ page }) => {
  await exhaustPool(page);
  await expect(page.getByTestId('pool-exhaustion')).toBeVisible();
});

test('pool exhaustion screen shows the submission form inline', async ({ page }) => {
  await exhaustPool(page);
  await expect(page.getByTestId('option-a-input')).toBeVisible();
  await expect(page.getByTestId('option-b-input')).toBeVisible();
  await expect(page.getByTestId('submit-button')).toBeVisible();
});

test('submitting from the pool exhaustion screen shows a confirmation', async ({ page }) => {
  await exhaustPool(page);
  await page.getByTestId('option-a-input').fill('Été');
  await page.getByTestId('option-b-input').fill('Hiver');
  await page.getByTestId('submit-button').click();
  await expect(page.getByTestId('submission-success')).toBeVisible();
});
