import { Divider } from '@arco-design/web-react'

import KillPortTool from './KillPortTool'
import OpenWindow from './OpenWindow'
import GitSetting from './GitSetting'
import ExecCommand from './ExecCommand'

export default function SmallTool() {
  return (
    <div className="p-[40px]">
      <KillPortTool />

      <Divider />

      <OpenWindow />

      <Divider />

      <ExecCommand />

      {/* <GitSetting /> */}
    </div>
  )
}
