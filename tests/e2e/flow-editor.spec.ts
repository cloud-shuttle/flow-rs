import { test, expect } from '@playwright/test';

test.describe('Leptos Flow Editor', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    // Wait for WASM module to load
    await page.waitForFunction(() => {
      return window.simple_flow_example !== undefined;
    }, { timeout: 10000 });
  });

  test('should load the flow editor', async ({ page }) => {
    // Check that the canvas element exists
    const canvas = page.locator('#flow-canvas');
    await expect(canvas).toBeVisible();

    // Check that the canvas has the expected dimensions
    const canvasBox = await canvas.boundingBox();
    expect(canvasBox?.width).toBeGreaterThan(0);
    expect(canvasBox?.height).toBeGreaterThan(0);
  });

  test('should render initial nodes and edges', async ({ page }) => {
    const canvas = page.locator('#flow-canvas');

    // Take a screenshot to verify rendering
    await expect(canvas).toHaveScreenshot('initial-render.png');

    // Check that the canvas is not empty (has been drawn on)
    const canvasContent = await canvas.evaluate((el: HTMLCanvasElement) => {
      const ctx = el.getContext('2d');
      if (!ctx) return false;

      // Check if canvas has been drawn on by sampling pixels
      const imageData = ctx.getImageData(0, 0, el.width, el.height);
      const data = imageData.data;

      // Look for non-transparent pixels (indicating something was drawn)
      for (let i = 3; i < data.length; i += 4) {
        if (data[i] > 0) return true; // Alpha channel > 0
      }
      return false;
    });

    expect(canvasContent).toBe(true);
  });

  test('should handle mouse interactions', async ({ page }) => {
    const canvas = page.locator('#flow-canvas');

    // Test mouse click
    await canvas.click({ position: { x: 100, y: 100 } });

    // Test mouse drag
    await canvas.hover({ position: { x: 100, y: 100 } });
    await page.mouse.down();
    await page.mouse.move(200, 200);
    await page.mouse.up();

    // Verify no errors in console
    const errors: string[] = [];
    page.on('console', msg => {
      if (msg.type() === 'error') {
        errors.push(msg.text());
      }
    });

    // Wait a bit for any async operations
    await page.waitForTimeout(1000);

    // Check for critical errors (allow warnings)
    const criticalErrors = errors.filter(error =>
      !error.includes('warning') &&
      !error.includes('deprecated') &&
      !error.includes('non-passive')
    );

    expect(criticalErrors).toHaveLength(0);
  });

  test('should handle canvas panning', async ({ page }) => {
    const canvas = page.locator('#flow-canvas');

    // Test panning by dragging on empty space
    await canvas.hover({ position: { x: 50, y: 50 } });
    await page.mouse.down();
    await page.mouse.move(100, 100);
    await page.mouse.up();

    // Take screenshot after panning
    await expect(canvas).toHaveScreenshot('after-panning.png');
  });

  test('should be responsive', async ({ page }) => {
    // Test on mobile viewport
    await page.setViewportSize({ width: 375, height: 667 });

    const canvas = page.locator('#flow-canvas');
    await expect(canvas).toBeVisible();

    // Check that canvas adapts to mobile size
    const canvasBox = await canvas.boundingBox();
    expect(canvasBox?.width).toBeLessThanOrEqual(375);
    expect(canvasBox?.height).toBeLessThanOrEqual(667);
  });

  test('should handle performance under load', async ({ page }) => {
    const canvas = page.locator('#flow-canvas');

    // Measure performance by checking frame rate
    const performanceMetrics = await page.evaluate(() => {
      return new Promise((resolve) => {
        let frameCount = 0;
        const startTime = performance.now();

        function countFrames() {
          frameCount++;
          if (performance.now() - startTime < 1000) {
            requestAnimationFrame(countFrames);
          } else {
            resolve({
              fps: frameCount,
              duration: performance.now() - startTime
            });
          }
        }

        requestAnimationFrame(countFrames);
      });
    });

    // Should maintain reasonable frame rate (at least 30 FPS)
    expect(performanceMetrics.fps).toBeGreaterThan(30);
  });

  test('should handle WASM module loading', async ({ page }) => {
    // Check that WASM module is loaded
    const wasmLoaded = await page.evaluate(() => {
      return typeof window.simple_flow_example !== 'undefined';
    });

    expect(wasmLoaded).toBe(true);

    // Check that the run function is available
    const runFunctionAvailable = await page.evaluate(() => {
      return typeof window.simple_flow_example.run === 'function';
    });

    expect(runFunctionAvailable).toBe(true);
  });
});
