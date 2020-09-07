import './main.scss'
;(async function main() {
  const app = await import('./wasm/todo-web-client.js')
  app.render(document.querySelector('#root'))
})()
