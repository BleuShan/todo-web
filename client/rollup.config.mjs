import {promises as fs} from 'fs'
import {dirname, resolve} from 'path'
import {fileURLToPath} from 'url'
import process from 'process'
import postcss from 'rollup-plugin-postcss'
import postcssConfig from './postcss.config.mjs'
import {terser} from 'rollup-plugin-terser'

const DIRNAME = dirname(fileURLToPath(import.meta.url))
const EXTERNALS = [/todo_web_client\.m?js/]
const WASM_EXT_REGEX = /_bg\.wasm$/

async function loadPkgJson() {
  const buffer = await fs.readFile(resolve(DIRNAME, 'package.json'))
  return JSON.parse(buffer.toString())
}

async function buildConfig() {
  const {main} = await loadPkgJson()
  const {TODO_WEB_ASSETS_DIR, TODO_WEB_WASM_BINDGEN_OUT_FILE} = process.env
  const plugins = [terser()]
  const wasmjsfile = TODO_WEB_WASM_BINDGEN_OUT_FILE.replace(
    WASM_EXT_REGEX,
    '.js'
  )

  return [
    {
      input: resolve(DIRNAME, 'src', main),
      external: (id) => EXTERNALS.some((pattern) => pattern.test(id)),
      output: {
        file: resolve(TODO_WEB_ASSETS_DIR, main),
        format: 'esm'
      },
      plugins: [
        postcss({
          extract: true,
          ...postcssConfig
        }),
        ...plugins
      ]
    },
    {
      input: wasmjsfile,
      output: {
        file: wasmjsfile,
        format: 'esm'
      },
      plugins
    }
  ]
}
export default buildConfig()
