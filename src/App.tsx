import { invoke } from '@tauri-apps/api/core'
import './App.css'

function App() {
  const checkUpdateRs = () => {
    invoke('check_update')
  }

  return (
    <main className="container">
      <div>
        <button onClick={checkUpdateRs}>检查更新 by rust</button>
      </div>
    </main>
  )
}

export default App
