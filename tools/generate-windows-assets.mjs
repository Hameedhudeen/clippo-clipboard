#!/usr/bin/env node
import { deflateSync } from "node:zlib";
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const outputDir = join(process.cwd(), "packaging", "windows", "Assets");

const assets = [
  ["StoreLogo.png", 50],
  ["Square44x44Logo.png", 44],
  ["Square150x150Logo.png", 150],
];

function crc32(buffer) {
  let crc = 0xffffffff;
  for (const byte of buffer) {
    crc ^= byte;
    for (let bit = 0; bit < 8; bit += 1) {
      crc = (crc >>> 1) ^ (0xedb88320 & -(crc & 1));
    }
  }
  return (crc ^ 0xffffffff) >>> 0;
}

function chunk(type, data) {
  const typeBuffer = Buffer.from(type, "ascii");
  const length = Buffer.alloc(4);
  length.writeUInt32BE(data.length);
  const checksum = Buffer.alloc(4);
  checksum.writeUInt32BE(crc32(Buffer.concat([typeBuffer, data])));
  return Buffer.concat([length, typeBuffer, data, checksum]);
}

function putPixel(buffer, width, x, y, [red, green, blue, alpha]) {
  if (x < 0 || x >= width || y < 0 || y >= width) {
    return;
  }
  const index = y * (width * 4 + 1) + 1 + x * 4;
  buffer[index] = red;
  buffer[index + 1] = green;
  buffer[index + 2] = blue;
  buffer[index + 3] = alpha;
}

function fillRect(buffer, width, x, y, rectWidth, rectHeight, color) {
  for (let row = y; row < y + rectHeight; row += 1) {
    for (let column = x; column < x + rectWidth; column += 1) {
      putPixel(buffer, width, column, row, color);
    }
  }
}

function drawLogo(size) {
  const stride = size * 4 + 1;
  const pixels = Buffer.alloc(stride * size);
  for (let y = 0; y < size; y += 1) {
    pixels[y * stride] = 0;
    for (let x = 0; x < size; x += 1) {
      const ratioX = x / Math.max(size - 1, 1);
      const ratioY = y / Math.max(size - 1, 1);
      const red = Math.round(37 - 17 * ratioY);
      const green = Math.round(99 + 85 * ratioX);
      const blue = Math.round(235 - 69 * ratioX);
      putPixel(pixels, size, x, y, [red, green, blue, 255]);
    }
  }

  const pad = Math.max(4, Math.round(size * 0.18));
  const boardX = Math.round(size * 0.28);
  const boardY = Math.round(size * 0.26);
  const boardW = size - boardX - pad;
  const boardH = size - boardY - Math.round(size * 0.16);
  const clipW = Math.round(size * 0.34);
  const clipH = Math.max(4, Math.round(size * 0.12));
  const clipX = Math.round((size - clipW) / 2);
  const clipY = Math.round(size * 0.14);

  fillRect(pixels, size, boardX, boardY, boardW, boardH, [248, 250, 252, 255]);
  fillRect(pixels, size, clipX, clipY, clipW, clipH, [255, 255, 255, 255]);

  const lineX = boardX + Math.round(boardW * 0.16);
  const lineY = boardY + Math.round(boardH * 0.26);
  const lineW = Math.round(boardW * 0.66);
  const lineH = Math.max(2, Math.round(size * 0.05));
  fillRect(pixels, size, lineX, lineY, lineW, lineH, [30, 41, 59, 220]);
  fillRect(pixels, size, lineX, lineY + lineH * 3, Math.round(lineW * 0.72), lineH, [30, 41, 59, 180]);
  fillRect(pixels, size, lineX, lineY + lineH * 6, Math.round(lineW * 0.88), lineH, [30, 41, 59, 140]);

  return pixels;
}

function png(size) {
  const signature = Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]);
  const header = Buffer.alloc(13);
  header.writeUInt32BE(size, 0);
  header.writeUInt32BE(size, 4);
  header[8] = 8;
  header[9] = 6;
  header[10] = 0;
  header[11] = 0;
  header[12] = 0;

  return Buffer.concat([
    signature,
    chunk("IHDR", header),
    chunk("IDAT", deflateSync(drawLogo(size), { level: 9 })),
    chunk("IEND", Buffer.alloc(0)),
  ]);
}

mkdirSync(outputDir, { recursive: true });
for (const [name, size] of assets) {
  writeFileSync(join(outputDir, name), png(size));
}
