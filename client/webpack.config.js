const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const path = require('path')

module.exports = {
  mode: 'production',
  entry: path.resolve(__dirname, 'main.js'),
  module: {
    rules: [
      {
        test: /\.s?css$/,
        use: [
          MiniCssExtractPlugin.loader,
          {
            loader: 'css-loader',
            options: {importLoaders: 1}
          },
          'postcss-loader'
        ]
      }
    ]
  },
  output: {
    path: path.resolve(__dirname, '..', 'assets')
  },
  plugins: [
    new MiniCssExtractPlugin({filename: '[name].[hash].css'}),
    new HtmlWebpackPlugin({
      template: path.resolve(__dirname, 'index.ejs')
    }),
    new WasmPackPlugin({
      crateDirectory: __dirname,
      outDir: path.resolve(__dirname, 'wasm'),
      outName: 'todo-web-client'
    })
  ]
}
