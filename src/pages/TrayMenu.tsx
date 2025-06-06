import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
import { useEffect } from 'react'

export default function TrayMenu() {
  useEffect(() => {
    const appWindow = getCurrentWebviewWindow()

    appWindow.onFocusChanged(focused => {
      if (!focused) {
        appWindow.hide()
      }
    })
  }, [])

  return <div>TrayMenu</div>
}
