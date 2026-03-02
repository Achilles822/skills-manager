import sharp from 'sharp';
import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const iconsDir = join(__dirname, '..', 'src-tauri', 'icons');
const svgPath = join(iconsDir, 'icon.svg');
const svgBuffer = readFileSync(svgPath);

const sizes = [
  { name: 'icon.png', size: 512 },
  { name: '32x32.png', size: 32 },
  { name: '128x128.png', size: 128 },
  { name: '128x128@2x.png', size: 256 },
  { name: 'Square30x30Logo.png', size: 30 },
  { name: 'Square44x44Logo.png', size: 44 },
  { name: 'Square71x71Logo.png', size: 71 },
  { name: 'Square89x89Logo.png', size: 89 },
  { name: 'Square107x107Logo.png', size: 107 },
  { name: 'Square142x142Logo.png', size: 142 },
  { name: 'Square150x150Logo.png', size: 150 },
  { name: 'Square284x284Logo.png', size: 284 },
  { name: 'Square310x310Logo.png', size: 310 },
  { name: 'StoreLogo.png', size: 50 },
];

for (const { name, size } of sizes) {
  await sharp(svgBuffer)
    .resize(size, size)
    .png()
    .toFile(join(iconsDir, name));
  console.log(`Generated ${name} (${size}x${size})`);
}

// Generate ICO with multiple sizes embedded
const icoSizes = [16, 24, 32, 48, 64, 128, 256];
const icoBuffers = [];

for (const size of icoSizes) {
  const buf = await sharp(svgBuffer)
    .resize(size, size)
    .png()
    .toBuffer();
  icoBuffers.push({ size, buf });
}

function createIco(images) {
  const headerSize = 6;
  const dirEntrySize = 16;
  const numImages = images.length;
  let offset = headerSize + dirEntrySize * numImages;

  const header = Buffer.alloc(headerSize);
  header.writeUInt16LE(0, 0);
  header.writeUInt16LE(1, 2);
  header.writeUInt16LE(numImages, 4);

  const dirEntries = [];
  for (const img of images) {
    const entry = Buffer.alloc(dirEntrySize);
    const w = img.size >= 256 ? 0 : img.size;
    const h = img.size >= 256 ? 0 : img.size;
    entry.writeUInt8(w, 0);
    entry.writeUInt8(h, 1);
    entry.writeUInt8(0, 2);
    entry.writeUInt8(0, 3);
    entry.writeUInt16LE(1, 4);
    entry.writeUInt16LE(32, 6);
    entry.writeUInt32LE(img.buf.length, 8);
    entry.writeUInt32LE(offset, 12);
    offset += img.buf.length;
    dirEntries.push(entry);
  }

  const buffers = [header, ...dirEntries, ...images.map(i => i.buf)];
  return Buffer.concat(buffers);
}

const icoBuffer = createIco(icoBuffers);
writeFileSync(join(iconsDir, 'icon.ico'), icoBuffer);
console.log('Generated icon.ico');

// Generate ICNS (simplified - use largest PNG wrapped)
// Tauri on macOS uses icon.icns. We'll create a basic one with the 512px icon.
const png512 = await sharp(svgBuffer).resize(512, 512).png().toBuffer();
const png256 = await sharp(svgBuffer).resize(256, 256).png().toBuffer();
const png1024 = await sharp(svgBuffer).resize(1024, 1024).png().toBuffer();

function createIcns(images) {
  const entries = [];
  const types = [
    { type: 'ic08', buf: images.find(i => i.size === 256)?.buf },
    { type: 'ic09', buf: images.find(i => i.size === 512)?.buf },
    { type: 'ic10', buf: images.find(i => i.size === 1024)?.buf },
  ].filter(e => e.buf);

  let totalSize = 8;
  for (const e of types) {
    totalSize += 8 + e.buf.length;
  }

  const header = Buffer.alloc(8);
  header.write('icns', 0, 4, 'ascii');
  header.writeUInt32BE(totalSize, 4);
  entries.push(header);

  for (const e of types) {
    const entryHeader = Buffer.alloc(8);
    entryHeader.write(e.type, 0, 4, 'ascii');
    entryHeader.writeUInt32BE(8 + e.buf.length, 4);
    entries.push(entryHeader);
    entries.push(e.buf);
  }

  return Buffer.concat(entries);
}

const icnsBuffer = createIcns([
  { size: 256, buf: png256 },
  { size: 512, buf: png512 },
  { size: 1024, buf: png1024 },
]);
writeFileSync(join(iconsDir, 'icon.icns'), icnsBuffer);
console.log('Generated icon.icns');

console.log('\nAll icons generated successfully!');
