const fs = require("node:fs");
const path = require("node:path");

const APP_TITLE = "Studio Display Brightness";

function normalizeVersionTag(tag) {
  const value = String(tag ?? "").trim();
  const match = /^v(\d+\.\d+\.\d+)$/.exec(value);

  if (!match) {
    throw new Error(`Expected version tag like v1.2.3, got "${value || "<empty>"}"`);
  }

  return match[1];
}

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function writeJson(filePath, data) {
  fs.writeFileSync(filePath, `${JSON.stringify(data, null, 2)}\n`);
}

function updateCargoPackageVersion(cargoToml, version) {
  const lines = cargoToml.split(/\r?\n/);
  let inPackageSection = false;
  let updated = false;

  const nextLines = lines.map((line) => {
    if (/^\[package\]\s*$/.test(line)) {
      inPackageSection = true;
      return line;
    }

    if (/^\[.+\]\s*$/.test(line)) {
      inPackageSection = false;
      return line;
    }

    if (inPackageSection && /^version\s*=/.test(line)) {
      updated = true;
      return `version = "${version}"`;
    }

    return line;
  });

  if (!updated) {
    throw new Error("Could not find [package] version in src-tauri/Cargo.toml");
  }

  return nextLines.join("\n");
}

function updateCargoLockPackageVersion(cargoLock, packageName, version) {
  const lines = cargoLock.split(/\r?\n/);
  let inPackageBlock = false;
  let packageBlockMatches = false;
  let updated = false;

  const nextLines = lines.map((line) => {
    if (/^\[\[package\]\]\s*$/.test(line)) {
      inPackageBlock = true;
      packageBlockMatches = false;
      return line;
    }

    if (inPackageBlock && /^name\s*=/.test(line)) {
      packageBlockMatches = line === `name = "${packageName}"`;
      return line;
    }

    if (inPackageBlock && packageBlockMatches && /^version\s*=/.test(line)) {
      updated = true;
      packageBlockMatches = false;
      return `version = "${version}"`;
    }

    return line;
  });

  if (!updated) {
    throw new Error(`Could not find ${packageName} version in src-tauri/Cargo.lock`);
  }

  return nextLines.join("\n");
}

function updateHtmlTitle(html, title) {
  const nextHtml = html.replace(/<title>[^<]*<\/title>/, `<title>${title}</title>`);

  if (nextHtml === html) {
    throw new Error("Could not find <title> in index.html");
  }

  return nextHtml;
}

function syncReleaseVersion(rootDir, tag) {
  const version = normalizeVersionTag(tag);
  const title = `${APP_TITLE} v${version}`;

  const packageJsonPath = path.join(rootDir, "package.json");
  const packageJson = readJson(packageJsonPath);
  packageJson.version = version;
  writeJson(packageJsonPath, packageJson);

  const packageLockPath = path.join(rootDir, "package-lock.json");
  if (fs.existsSync(packageLockPath)) {
    const packageLock = readJson(packageLockPath);
    packageLock.version = version;

    if (packageLock.packages?.[""]) {
      packageLock.packages[""].version = version;
    }

    writeJson(packageLockPath, packageLock);
  }

  const cargoTomlPath = path.join(rootDir, "src-tauri", "Cargo.toml");
  const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
  fs.writeFileSync(cargoTomlPath, updateCargoPackageVersion(cargoToml, version));

  const cargoLockPath = path.join(rootDir, "src-tauri", "Cargo.lock");
  if (fs.existsSync(cargoLockPath)) {
    const cargoLock = fs.readFileSync(cargoLockPath, "utf8");
    fs.writeFileSync(
      cargoLockPath,
      updateCargoLockPackageVersion(cargoLock, packageJson.name, version),
    );
  }

  const tauriConfigPath = path.join(rootDir, "src-tauri", "tauri.conf.json");
  const tauriConfig = readJson(tauriConfigPath);
  tauriConfig.version = version;

  if (!Array.isArray(tauriConfig.app?.windows) || tauriConfig.app.windows.length === 0) {
    throw new Error("Could not find app.windows[0] in src-tauri/tauri.conf.json");
  }

  tauriConfig.app.windows[0].title = title;
  writeJson(tauriConfigPath, tauriConfig);

  const indexPath = path.join(rootDir, "index.html");
  const html = fs.readFileSync(indexPath, "utf8");
  fs.writeFileSync(indexPath, updateHtmlTitle(html, title));

  return { title, version };
}

function runCli() {
  const tag = process.argv[2] || process.env.RELEASE_VERSION_TAG || process.env.GITHUB_REF_NAME;
  const rootDir = path.resolve(__dirname, "..");
  const result = syncReleaseVersion(rootDir, tag);
  console.log(`Synchronized release version ${result.version}`);
  console.log(`Title: ${result.title}`);
}

if (require.main === module) {
  try {
    runCli();
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}

module.exports = {
  normalizeVersionTag,
  syncReleaseVersion,
};
