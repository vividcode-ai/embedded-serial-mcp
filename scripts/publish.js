const { execSync } = require('child_process');
const { join } = require('path');

const packages = [
  'darwin-x64',
  'darwin-arm64',
  'linux-x64',
  'linux-arm64',
  'win32-x64',
  'cli',
];

for (const pkg of packages) {
  const dir = join(__dirname, '..', 'packages', pkg);
  console.log(`\n--- Publishing packages/${pkg} ---`);
  try {
    execSync('npm publish --access public', { cwd: dir, stdio: 'inherit' });
    console.log(`✓ packages/${pkg} published`);
  } catch (err) {
    console.error(`✗ packages/${pkg} failed: ${err.message}`);
    process.exit(1);
  }
}

console.log('\n✓ All packages published successfully');
