const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

/**@type {import("webpack").Configuration} */
module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.css$/i,
        use: ['style-loader', 'css-loader'],
      },
    ]
  },
  plugins: [
    new CopyWebpackPlugin(['index.html'])
  ],
};
