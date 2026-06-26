import { test, expect } from './fixtures';

test('home page shows a question with two options', async ({ page }) => {
  await page.goto('/');
  await expect(page.getByTestId('question')).toBeVisible();
  await expect(page.getByTestId('option-a')).toBeVisible();
  await expect(page.getByTestId('option-b')).toBeVisible();
});

test('voting on an option reveals the result distribution', async ({ page }) => {
  await page.goto('/');
  await page.getByTestId('option-a').click();
  await expect(page.getByTestId('results')).toBeVisible();
  await expect(page.getByTestId('result-a-count')).toBeVisible();
  await expect(page.getByTestId('result-b-count')).toBeVisible();
});

test('a next button appears after voting', async ({ page }) => {
  await page.goto('/');
  await page.getByTestId('option-a').click();
  await expect(page.getByTestId('next-button')).toBeVisible();
});

test('a voted question is not shown again in the same session', async ({ page }) => {
  await page.goto('/');
  const firstText = await page.getByTestId('question').textContent();
  await page.getByTestId('option-a').click();
  await page.getByTestId('next-button').click();
  const secondText = await page.getByTestId('question').textContent();
  expect(secondText).not.toBe(firstText);
});

test('skipping shows the next question without revealing results', async ({ page }) => {
  await page.goto('/');
  await page.getByTestId('skip-button').click();
  await expect(page.getByTestId('question')).toBeVisible();
  await expect(page.getByTestId('results')).not.toBeVisible();
});

test('a skipped question is not shown again in the same session', async ({ page }) => {
  await page.goto('/');
  const firstText = await page.getByTestId('question').textContent();
  await page.getByTestId('skip-button').click();
  const secondText = await page.getByTestId('question').textContent();
  expect(secondText).not.toBe(firstText);
});
