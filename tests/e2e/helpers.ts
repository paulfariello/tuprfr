import { readFileSync } from 'fs';
import { Client } from 'pg';
import { DB_URL } from './config';

async function withDb<T>(fn: (client: Client) => Promise<T>): Promise<T> {
  const client = new Client({ connectionString: DB_URL });
  await client.connect();
  try {
    return await fn(client);
  } finally {
    await client.end();
  }
}

export async function seedDb(): Promise<void> {
  await withDb(c => c.query(readFileSync('seed.sql', 'utf-8')));
}

export async function cleanDb(): Promise<void> {
  await withDb(c => c.query('TRUNCATE votes, questions, options CASCADE'));
}