import { test, expect } from './fixtures';

test('submit form has two option fields and a submit button', async ({ page }) => {
  await page.goto('/questions/new');
  await expect(page.getByTestId('option-a-input')).toBeVisible();
  await expect(page.getByTestId('option-b-input')).toBeVisible();
  await expect(page.getByTestId('submit-button')).toBeVisible();
});

test('submitting a valid question shows a confirmation', async ({ page }) => {
  await page.goto('/questions/new');
  await page.getByTestId('option-a-input').fill('Pizza');
  await page.getByTestId('option-b-input').fill('Sushi');
  await page.getByTestId('submit-button').click();
  await expect(page.getByTestId('submission-success')).toBeVisible();
});

test('submitting with an empty option shows a validation error', async ({ page }) => {
  await page.goto('/questions/new');
  await page.getByTestId('option-a-input').fill('Pizza');
  await page.getByTestId('submit-button').click();
  await expect(page.getByTestId('submission-error')).toBeVisible();
});

test('submitting with both options empty shows a validation error', async ({ page }) => {
  await page.goto('/questions/new');
  await page.getByTestId('submit-button').click();
  await expect(page.getByTestId('submission-error')).toBeVisible();
});
