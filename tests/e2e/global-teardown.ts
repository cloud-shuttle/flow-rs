import { FullConfig } from '@playwright/test';

async function globalTeardown(config: FullConfig) {
  console.log('🧹 Cleaning up E2E test environment...');

  // Add any cleanup logic here
  // For example, stopping servers, cleaning up files, etc.

  console.log('✅ E2E test environment cleaned up');
}

export default globalTeardown;
