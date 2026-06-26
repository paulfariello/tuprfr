import { test as base, expect } from '@playwright/test';
import { ChildProcess, spawn } from 'child_process';
import { DB_URL, SERVER_URL } from './config';
import { cleanDb, seedDb } from './helpers';

async function waitForServer(): Promise<void> {
  for (let i = 0; i < 100; i++) {
    try {
      await fetch(SERVER_URL);
      return;
    } catch {
      await new Promise(r => setTimeout(r, 100));
    }
  }
  throw new Error('tuprfr server did not become ready within 10s');
}

async function startServer(): Promise<ChildProcess> {
  const server = spawn('./target/debug/tuprfr', {
    env: { ...process.env, DATABASE_URL: DB_URL },
    stdio: 'ignore',
  });
  await waitForServer();
  return server;
}

export const test = base.extend<{ appServer: ChildProcess }>({
  appServer: [
    async ({}, use) => {
      await cleanDb();
      await seedDb();
      const server = await startServer();

      await use(server);

      server.kill('SIGKILL');
      await new Promise(resolve => server.on('exit', resolve));
      await cleanDb();
    },
    { auto: true },
  ],
});

export { expect };