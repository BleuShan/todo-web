import './stylesheets/main.scss'
import init, {render} from './todo_web_client.js'

async function main() {
  await init()
  render('#root')
}

window.addEventListener('load', main, {once: true})
