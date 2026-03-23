import OpenFolder from './pages/OpenFolder'
import Setting from './pages/Setting'
import QuickInput from './pages/QuickInput'
import { useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import TrayMenu from './pages/TrayMenu'
import { ConfigProvider } from '@arco-design/web-react'

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
    <ConfigProvider componentConfig={{ Button: { type: 'primary' }, Form: { autoComplete: 'off' } }}>
      {hash === 'openFolder' && <OpenFolder />}
      {hash === 'setting' && <Setting />}
      {hash === 'quickInput' && <QuickInput />}
      {hash === 'trayMenu' && <TrayMenu />}
    </ConfigProvider>
  )
}

export default App
