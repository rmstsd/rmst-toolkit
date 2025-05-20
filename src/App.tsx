import { useState } from 'react'
import reactLogo from './assets/react.svg'
import { invoke } from '@tauri-apps/api/core'
import { check } from '@tauri-apps/plugin-updater'
import { relaunch } from '@tauri-apps/plugin-process'
import './App.css'

function App() {
  const [greetMsg, setGreetMsg] = useState('')
  const [name, setName] = useState('')

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setGreetMsg(await invoke('greet', { name }))
  }

  const checkUpdate = async () => {
    const update = await check()
    console.log('update', update)
    if (update) {
      console.log(`found update ${update.version} from ${update.date} with notes ${update.body}`)
      let downloaded = 0
      let contentLength = 0
      // alternatively we could also call update.download() and update.install() separately
      await update.downloadAndInstall(event => {
        switch (event.event) {
          case 'Started':
            contentLength = event.data.contentLength
            console.log(`started downloading ${event.data.contentLength} bytes`)
            break
          case 'Progress':
            downloaded += event.data.chunkLength
            console.log(`downloaded ${downloaded} from ${contentLength}`)
            break
          case 'Finished':
            console.log('download finished')
            break
        }
      })

      console.log('update installed')
      await relaunch()
    }
  }

  const checkUpdateRs = () => {
    invoke('check_update')
  }

  return (
    <main className="container">
      <div>
        <button onClick={checkUpdate}>检查更新</button>
        <button onClick={checkUpdateRs}>检查更新rs</button>
      </div>

      <h1>Welcome to Tauri + React</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className="row"
        onSubmit={e => {
          e.preventDefault()
          greet()
        }}
      >
        <input id="greet-input" onChange={e => setName(e.currentTarget.value)} placeholder="Enter a name..." />
        <button type="submit">Greet</button>
      </form>
      <p>{greetMsg}</p>
    </main>
  )
}

export default App
