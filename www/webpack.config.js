const path = require('path');
const distPath = path.join(__dirname, "dist");

module.exports = {
  mode: "development",
  entry: path.join(distPath, "bootstrap.js"),
  output: {
    path: distPath,
    filename: "bootstrap.js",
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  },
  devServer: {
    static: {
      directory: distPath,
    },
    compress: true,
    port: 9000
  }
};
