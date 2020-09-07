const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin')
const HtmlWebpackPlugin = require('html-webpack-plugin')
const MiniCssExtractPlugin = require('mini-css-extract-plugin')
const {name: outName, main: entryFile} = require('./package.json')
const {resolve: resolvePath} = require('path')

function resolvePathFromDir(...args) {
  return resolvePath(__dirname, ...args)
}

module.exports = () => {
  const entry = resolvePathFromDir(entryFile)
  const output = {
    path: resolvePathFromDir('..', 'assets')
  }
  const wasmDir = resolvePathFromDir('wasm')
  return {
    mode: 'production',
    entry,
    module: {
      rules: [
        {
          test: /\.s?css$/,
          use: [
            MiniCssExtractPlugin.loader,
            {
              loader: 'css-loader',
              options: {
                importLoaders: 2
              }
            },
            {
              loader: 'postcss-loader'
            },
            {
              loader: 'sass-loader',
              options: {
                implementation: require('sass'),
                sassOptions: {
                  fiber: require('fibers')
                }
              }
            }
          ]
        }
      ]
    },
    output,
    plugins: [
      new MiniCssExtractPlugin({filename: '[name].[hash].css'}),
      new HtmlWebpackPlugin({
        template: resolvePathFromDir('index.ejs')
      }),
      new WasmPackPlugin({
        crateDirectory: __dirname,
        outDir: wasmDir,
        extraArgs: '--no-typescript',
        outName
      })
    ]
  }
}
