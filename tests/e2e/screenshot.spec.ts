import type { Page } from '@playwright/test';
import { test } from './fixtures';

const states: { name: string; setup: (page: Page) => Promise<void> }[] = [
  {
    name: 'home',
    setup: async page => {
      await page.goto('/');
    },
  },
  {
    name: 'results',
    setup: async page => {
      await page.goto('/');
      await page.getByTestId('option-a').click();
    },
  },
  {
    name: 'pool-exhaustion',
    setup: async page => {
      await page.goto('/');
      await page.getByTestId('option-a').click();
      await page.getByTestId('next-button').click();
      await page.getByTestId('option-a').click();
      await page.getByTestId('next-button').click();
    },
  },
  {
    name: 'submit-form',
    setup: async page => {
      await page.goto('/questions/new');
    },
  },
  {
    name: 'submit-success',
    setup: async page => {
      await page.goto('/questions/new');
      await page.getByTestId('option-a-input').fill('Pizza');
      await page.getByTestId('option-b-input').fill('Sushi');
      await page.getByTestId('submit-button').click();
    },
  },
  {
    name: 'submit-error',
    setup: async page => {
      await page.goto('/questions/new');
      await page.getByTestId('option-a-input').fill('Pizza');
      await page.getByTestId('submit-button').click();
    },
  },
];

for (const { name, setup } of states) {
  test(`screenshot: ${name}`, async ({ page }) => {
    await setup(page);
    await page.screenshot({ path: `test-results/screenshot-${name}.png`, fullPage: true });
  });
}
