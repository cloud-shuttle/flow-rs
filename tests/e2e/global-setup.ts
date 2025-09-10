import { chromium, FullConfig } from '@playwright/test';

async function globalSetup(config: FullConfig) {
  console.log('🚀 Setting up E2E test environment...');

  // Build WASM module if needed
  const { execSync } = require('child_process');

  try {
    console.log('📦 Building WASM module...');
    execSync('cd examples/simple-flow && wasm-pack build --target web --out-dir pkg --dev', {
      stdio: 'inherit',
      timeout: 120000 // 2 minutes
    });
    console.log('✅ WASM module built successfully');
  } catch (error) {
    console.error('❌ Failed to build WASM module:', error);
    throw error;
  }

  // Test that the server is accessible
  const browser = await chromium.launch();
  const page = await browser.newPage();

  try {
    await page.goto('http://localhost:8080', {
      waitUntil: 'networkidle',
      timeout: 30000
    });
    console.log('✅ Test server is accessible');
  } catch (error) {
    console.error('❌ Test server is not accessible:', error);
    throw error;
  } finally {
    await browser.close();
  }

  console.log('🎯 E2E test environment ready!');
}

export default globalSetup;
