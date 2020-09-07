import './main.scss'
;(async function main() {
  const app = await import('./wasm/todo-web-client')
  app.render(document.querySelector('#root'))
})()
