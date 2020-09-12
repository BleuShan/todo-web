import {promises as fs} from 'fs'
import {dirname, resolve} from 'path'
import {fileURLToPath} from 'url'
import process from 'process'
import postcss from 'rollup-plugin-postcss'
import postcssConfig from './postcss.config.mjs'
import {terser} from 'rollup-plugin-terser'
import favicons from 'rollup-plugin-favicons'

const DIRNAME = dirname(fileURLToPath(import.meta.url))

async function loadPkgJson() {
  const buffer = await fs.readFile(resolve(DIRNAME, 'package.json'))
  return JSON.parse(buffer.toString())
}

async function buildConfig() {
  const {main} = await loadPkgJson()
  const {TODO_WEB_ASSETS_DIR, CARGO_MAKE_PROJECT_VERSION_MEMBER} = process.env

  return {
    input: main,
    output: {
      dir: TODO_WEB_ASSETS_DIR,
      preserveModules: true,
      format: 'esm',
      entryFileNames: '[name].mjs'
    },
    plugins: [
      favicons.default({
        source: resolve(DIRNAME, 'assets', 'favicon.png'),
        configuration: {
          appName: CARGO_MAKE_PROJECT_VERSION_MEMBER,
          developerName: 'Bleushan',
          start_url: '/'
        }
      }),
      postcss({
        extract: true,
        ...postcssConfig
      }),
      terser()
    ]
  }
}
export default buildConfig()
