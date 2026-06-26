import { defineConfig } from '@playwright/test';
import { SERVER_URL } from './tests/e2e/config';

export default defineConfig({
  testDir: 'tests/e2e',
  globalSetup: './tests/e2e/global-setup.ts',
  globalSetupTimeout: 5 * 60 * 1000,
  fullyParallel: false,
  workers: 1,
  use: {
    baseURL: SERVER_URL,
    trace: 'on-first-retry',
  },
  projects: [
    { name: 'chromium', use: { browserName: 'chromium' } },
  ],
});