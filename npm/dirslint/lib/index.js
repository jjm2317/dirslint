#!/usr/bin/env node

const {spawnSync} = require( "child_process");

/**
 * Returns the executable path which is located inside `node_modules`
 * The naming convention is dirslint-${os}-${arch}
 * If the platform is `win32` or `cygwin`, executable will include a `.exe` extension.
 * @see https://nodejs.org/api/os.html#osarch
 * @see https://nodejs.org/api/os.html#osplatform
 * @example "x/xx/node_modules/dirslint-darwin-arm64"
 */
function getExePath() {
  const arch = "arm64";
  let os = "darwin";
  let extension = "";
  // todo: add support for other platforms

  try {
    // Since the binary will be located inside `node_modules`, we can simply call `require.resolve`
    return require.resolve(`dirslint-${os}-${arch}/bin/dirslint${extension}`);
  } catch (e) {
    throw new Error(
      `Couldn't find application binary inside node_modules for ${os}-${arch}`
    );
  }
}


/**
 * Runs the application with args using nodejs spawn
 */
function run() {
  const args = process.argv.slice(2);
  const processResult = spawnSync(getExePath(), ["--"].concat(args), { stdio: "inherit" });
  process.exit(processResult.status ?? 0);
}

run();