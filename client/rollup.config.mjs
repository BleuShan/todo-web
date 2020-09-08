import {promises as fs} from 'fs'
import {dirname, resolve} from 'path'
import {fileURLToPath} from 'url'
import process from 'process'
import postcss from 'rollup-plugin-postcss'
import postcssConfig from './postcss.config.mjs'
import {terser} from 'rollup-plugin-terser'

const DIRNAME = dirname(fileURLToPath(import.meta.url))
const EXTERNALS = [/todo_web_client\.js/]

async function loadPkgJson() {
  const buffer = await fs.readFile(resolve(DIRNAME, 'package.json'))
  return JSON.parse(buffer.toString())
}

async function buildConfig() {
  const {main: input} = await loadPkgJson()
  const dir = process.env.TODO_WEB_ASSETS_DIR

  return {
    input,
    external: (id) => EXTERNALS.some((pattern) => pattern.test(id)),
    output: {
      dir,
      format: 'esm'
    },
    plugins: [
      postcss({
        extract: true,
        ...postcssConfig
      }),
      terser()
    ]
  }
}
export default buildConfig()
