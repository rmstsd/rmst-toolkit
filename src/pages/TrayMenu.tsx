import { Menu } from '@arco-design/web-react'
import { invoke } from '@tauri-apps/api/core'
import ResizeObserver from 'rc-resize-observer'
import { useFocus } from '../utils'

export default function TrayMenu() {
  useFocus(({ appWindow, focused }) => {
    if (!focused) {
      appWindow.hide()
    }
  })

  return (
    <ResizeObserver
      onResize={size => {
        console.log(size)
        invoke('setWindowSize', { label: 'trayMenu', width: size.width, height: size.height })
      }}
    >
      <div style={{ width: 100 }} className="tray-menu ">
        <Menu
          selectable={false}
          onClickMenuItem={key => {
            invoke('trayMenu', { menuKey: key })
          }}
        >
          <Menu.Item key="setting">设置</Menu.Item>
          <Menu.Item key="restart">重启</Menu.Item>
          <Menu.Item key="quit">退出</Menu.Item>
        </Menu>
      </div>
    </ResizeObserver>
  )
}
