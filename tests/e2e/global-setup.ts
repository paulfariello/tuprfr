import { execSync } from 'child_process';
import { Client } from 'pg';
import { CONTAINER, DB_URL, PG_PORT } from './config';

async function waitForPostgres(): Promise<void> {
  for (let i = 0; i < 30; i++) {
    try {
      const client = new Client({ connectionString: DB_URL });
      await client.connect();
      await client.end();
      return;
    } catch {
      await new Promise(r => setTimeout(r, 1000));
    }
  }
  throw new Error('PostgreSQL did not become ready within 30s');
}

export default async function globalSetup() {
  execSync(
    `podman rm -f ${CONTAINER} 2>/dev/null || true && \
     podman run -d --name ${CONTAINER} \
       -e POSTGRES_USER=tuprfr \
       -e POSTGRES_PASSWORD=tuprfr \
       -e POSTGRES_DB=tuprfr_test \
       -p ${PG_PORT}:5432 \
       postgres:16-alpine`,
    { stdio: 'inherit' },
  );

  await waitForPostgres();

  for (let i = 0; i < 10; i++) {
    try {
      execSync(`sqlx migrate run --database-url "${DB_URL}"`, { stdio: 'inherit' });
      break;
    } catch {
      if (i === 9) throw new Error('sqlx migrate run failed after 10 attempts');
      await new Promise(r => setTimeout(r, 1000));
    }
  }

  execSync('cargo build', { stdio: 'inherit' });

  return async () => {
    execSync(`podman stop ${CONTAINER} && podman rm ${CONTAINER}`, { stdio: 'ignore' });
  };
}