const { copyFileSync, mkdirSync, existsSync, chmodSync } = require('fs');
const { join } = require('path');
const { platform, arch } = process;

const platformMap = {
  'darwin-x64': 'darwin-x64',
  'darwin-arm64': 'darwin-arm64',
  'linux-x64': 'linux-x64',
  'linux-arm64': 'linux-arm64',
  'win32-x64': 'win32-x64',
};

const key = `${platform}-${arch}`;
const targetDir = platformMap[key];

if (!targetDir) {
  console.error(`Current platform ${key} is not a supported build target`);
  process.exit(1);
}

const ext = platform === 'win32' ? '.exe' : '';
const source = join(__dirname, '..', 'target', 'release', `embedded-serial-mcp${ext}`);
const destDir = join(__dirname, '..', 'packages', targetDir, 'bin');

if (!existsSync(source)) {
  console.error(`Binary not found at ${source}`);
  console.error('Run "cargo build --release" first');
  process.exit(1);
}

mkdirSync(destDir, { recursive: true });
copyFileSync(source, join(destDir, `embedded-serial-mcp${ext}`));

if (platform !== 'win32') {
  try {
    chmodSync(join(destDir, 'embedded-serial-mcp'), 0o755);
  } catch {}
}

console.log(`Copied binary to packages/${targetDir}/bin/`);
