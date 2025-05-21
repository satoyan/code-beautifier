// html2clip.js
// Use ESM imports for clipboardy (which is now ESM-only)
import puppeteer from 'puppeteer';
import clipboardy from 'clipboardy';
import path from 'path';
import { fileURLToPath } from 'url';
import fs from 'fs';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

(async () => {
  const htmlPath = process.argv[2];
  if (!htmlPath) {
    console.error('Usage: node html2clip.js <html-file>');
    process.exit(1);
  }
  const absPath = path.resolve(htmlPath);
  const browser = await puppeteer.launch({ headless: 'new' });
  const page = await browser.newPage();
  await page.goto('file://' + absPath, { waitUntil: 'networkidle0' });
  await page.waitForSelector('.code-window');
  const codeWindow = await page.$('.code-window');
  const imageBuffer = await codeWindow.screenshot({ type: 'png' });
  await clipboardy.write(imageBuffer);
  await browser.close();
  console.log('Image copied to clipboard!');
})();
