import OpenFolder from './pages/OpenFolder'
import Setting from './pages/Setting'
import QuickInput from './pages/QuickInput'
import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import TrayMenu from './pages/TrayMenu'

function App() {
  const hash = location.hash.slice(1)

  useEffect(() => {
    document.addEventListener('keydown', evt => {
      if (evt.key === 'Escape') {
        invoke('hideWindow')
      }
    })
  }, [])

  return (
    <>
      {hash === 'openFolder' && <OpenFolder />}
      {hash === 'setting' && <Setting />}
      {hash === 'quickInput' && <QuickInput />}
      {hash === 'trayMenu' && <TrayMenu />}
    </>
  )
}

export default App
