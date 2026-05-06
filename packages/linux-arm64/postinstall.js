#!/usr/bin/env node
const fs = require('fs');
const path = require('path');
const bin = path.join(__dirname, 'bin', 'embedded-serial-mcp');
if (process.platform !== 'win32') {
  try { fs.chmodSync(bin, 0o755); } catch {}
}
