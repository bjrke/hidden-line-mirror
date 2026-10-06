const path = require("path");
const { execSync } = require("child_process");
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

function git(args, fallback) {
  try {
    return execSync(`git ${args}`, { cwd: __dirname }).toString().trim();
  } catch (e) {
    return fallback;
  }
}

const commit = process.env.CI_COMMIT_SHORT_SHA || git("rev-parse --short HEAD", "unknown");
const branch = process.env.CI_COMMIT_REF_NAME || git("rev-parse --abbrev-ref HEAD", "unknown");
const time = new Date().toISOString().replace(/\.\d+Z$/, "Z");
const buildInfo = `build ${time} · ${commit} · ${branch}`;

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js",
    clean: true
  },
  devServer: {
    static: {
      directory: dist
    }
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  plugins: [
    new webpack.DefinePlugin({
      __BUILD_INFO__: JSON.stringify(buildInfo)
    }),

    new CopyPlugin( {
      patterns: [
        path.resolve(__dirname, "static"),
        { from: path.resolve(__dirname, "LICENSE"), to: "LICENSE.txt" }
      ]
    }),

    new WasmPackPlugin({
      crateDirectory: __dirname
    }),
  ]
};
