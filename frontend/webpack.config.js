const path = require("path");
const webpack = require("webpack");
const CopyPlugin = require("copy-webpack-plugin");
let WasmPackPlugin;
if (process.env.CI) {
  console.log("CI detected, skipping wasm-pack plugin");
} else {
  WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
}

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: ["./ts/index.ts", "./pkg/index.js"],
  devtool: 'inline-source-map',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    path: dist,
    filename: "index.bundle.js"
  },
  devServer: {
    static: {
      directory: path.join(__dirname, 'static'),
    },
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static"),
    ]),

    ...(process.env.CI ? [] : [
      new WasmPackPlugin({
        crateDirectory: __dirname,
      }),]),

    new webpack.DefinePlugin({
      "process.env.API_URL": JSON.stringify(process.env.API_URL || "http://192.168.178.127:5000/api/v1"),
    })

  ],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  performance: {
    hints: false
  }
};

