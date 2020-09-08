import './stylesheets/main.scss'
import loadWasm from './todo_web_client.js'
;(async function main() {
  const app = await loadWasm()
  app.render('#root')
})()
