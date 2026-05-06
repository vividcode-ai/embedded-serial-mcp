const { readFileSync, writeFileSync } = require('fs');
const { join } = require('path');

const newVersion = process.argv[2];
if (!newVersion || !/^\d+\.\d+\.\d+/.test(newVersion)) {
  console.error('Usage: node scripts/set-version.js 0.0.1');
  process.exit(1);
}

const root = join(__dirname, '..');

const cargoPath = join(root, 'Cargo.toml');
let cargo = readFileSync(cargoPath, 'utf8');
cargo = cargo.replace(/^(version\s*=\s*")[^"]+(")/m, `$1${newVersion}$2`);
writeFileSync(cargoPath, cargo);
console.log(`Cargo.toml -> ${newVersion}`);

const pkgDirs = [
  'darwin-x64', 'darwin-arm64',
  'linux-x64', 'linux-arm64',
  'win32-x64', 'cli',
];

for (const dir of pkgDirs) {
  const pkgPath = join(root, 'packages', dir, 'package.json');
  const pkg = JSON.parse(readFileSync(pkgPath, 'utf8'));
  pkg.version = newVersion;

  if (pkg.optionalDependencies) {
    for (const key of Object.keys(pkg.optionalDependencies)) {
      if (key.startsWith('@vividcodeai/embedded-serial-mcp-')) {
        pkg.optionalDependencies[key] = newVersion;
      }
    }
  }

  writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n');
  console.log(`packages/${dir}/package.json -> ${newVersion}`);
}

console.log('\nAll versions updated to', newVersion);
