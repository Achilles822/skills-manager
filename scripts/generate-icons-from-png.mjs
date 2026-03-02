import sharp from 'sharp';
import { readFileSync, writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const iconsDir = join(__dirname, '..', 'src-tauri', 'icons');
const sourcePath = process.argv[2];

if (!sourcePath) {
  console.error('Usage: node generate-icons-from-png.mjs <source-image>');
  process.exit(1);
}

const sourceBuffer = readFileSync(sourcePath);

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
  await sharp(sourceBuffer)
    .resize(size, size, { fit: 'cover' })
    .png()
    .toFile(join(iconsDir, name));
  console.log(`Generated ${name} (${size}x${size})`);
}

const icoSizes = [16, 24, 32, 48, 64, 128, 256];
const icoBuffers = [];
for (const size of icoSizes) {
  const buf = await sharp(sourceBuffer)
    .resize(size, size, { fit: 'cover' })
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
    entry.writeUInt8(img.size >= 256 ? 0 : img.size, 0);
    entry.writeUInt8(img.size >= 256 ? 0 : img.size, 1);
    entry.writeUInt8(0, 2);
    entry.writeUInt8(0, 3);
    entry.writeUInt16LE(1, 4);
    entry.writeUInt16LE(32, 6);
    entry.writeUInt32LE(img.buf.length, 8);
    entry.writeUInt32LE(offset, 12);
    offset += img.buf.length;
    dirEntries.push(entry);
  }
  return Buffer.concat([header, ...dirEntries, ...images.map(i => i.buf)]);
}

writeFileSync(join(iconsDir, 'icon.ico'), createIco(icoBuffers));
console.log('Generated icon.ico');

const png256 = await sharp(sourceBuffer).resize(256, 256, { fit: 'cover' }).png().toBuffer();
const png512 = await sharp(sourceBuffer).resize(512, 512, { fit: 'cover' }).png().toBuffer();
const png1024 = await sharp(sourceBuffer).resize(1024, 1024, { fit: 'cover' }).png().toBuffer();

function createIcns(images) {
  const types = [
    { type: 'ic08', buf: images.find(i => i.size === 256)?.buf },
    { type: 'ic09', buf: images.find(i => i.size === 512)?.buf },
    { type: 'ic10', buf: images.find(i => i.size === 1024)?.buf },
  ].filter(e => e.buf);

  let totalSize = 8;
  for (const e of types) totalSize += 8 + e.buf.length;

  const header = Buffer.alloc(8);
  header.write('icns', 0, 4, 'ascii');
  header.writeUInt32BE(totalSize, 4);

  const entries = [header];
  for (const e of types) {
    const eh = Buffer.alloc(8);
    eh.write(e.type, 0, 4, 'ascii');
    eh.writeUInt32BE(8 + e.buf.length, 4);
    entries.push(eh, e.buf);
  }
  return Buffer.concat(entries);
}

writeFileSync(join(iconsDir, 'icon.icns'), createIcns([
  { size: 256, buf: png256 },
  { size: 512, buf: png512 },
  { size: 1024, buf: png1024 },
]));
console.log('Generated icon.icns');
console.log('\nAll icons generated successfully!');
