import './main.scss'
;(async function main() {
  const app = await import('todo_web_client.js')
  app.render(document.querySelector('#root'))
})()
