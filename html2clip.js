const puppeteer = require('puppeteer');
const clipboardy = require('clipboardy');
const path = require('path');

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
  clipboardy.writeSync(imageBuffer);
  await browser.close();
  console.log('Image copied to clipboard!');
})();
