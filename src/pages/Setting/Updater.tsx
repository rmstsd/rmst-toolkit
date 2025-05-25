import { Channel, invoke } from '@tauri-apps/api/core'
import { Button, Form, Message, Modal, Progress, Typography } from '@arco-design/web-react'
import { useState } from 'react'

interface Updater {
  needUpdate: boolean
  current_version: string
  version: string
}
type DownloadEvent =
  | {
      event: 'Started'
      data: { url: string; downloadId: number; content_length: number }
    }
  | {
      event: 'Progress'
      data: { downloadId: number; chunk_length: number }
    }
  | {
      event: 'Finished'
      data: { downloadId: number }
    }

export default function Updater() {
  const [updaterVisible, setUpdaterVisible] = useState(false)
  const [updateInfo, setUpdateInfo] = useState({} as Updater)
  const [checkLoading, setCheckLoading] = useState(false)

  const [updaterProgress, setUpdaterProgress] = useState(0)
  const [downloadLoading, setDownloadLoading] = useState(false)

  const checkUpdate = () => {
    setCheckLoading(true)
    invoke<Updater>('checkUpdate')
      .then(data => {
        console.log(data)

        if (!data.needUpdate) {
          Message.info('已经是最新版本')
          return
        }

        setUpdateInfo(data)
        setUpdaterVisible(true)
      })
      .finally(() => {
        setCheckLoading(false)
      })
  }

  const downloadAndInstall = () => {
    setDownloadLoading(true)

    const onEvent = new Channel<DownloadEvent>()

    let contentLength = 0
    let downloadedLength = 0

    onEvent.onmessage = message => {
      switch (message.event) {
        case 'Started': {
          setUpdaterProgress(0)
          contentLength = message.data.content_length
          break
        }
        case 'Progress': {
          downloadedLength += message.data.chunk_length
          const percent = Math.round(downloadedLength / contentLength) * 100
          setUpdaterProgress(percent)
          break
        }
        case 'Finished': {
          setUpdaterProgress(100)
          break
        }
      }
    }

    invoke('download_and_install', { onEvent }).then(() => {
      Message.success({ content: '操作成功', position: 'bottom' })
      setUpdaterVisible(false)
      setDownloadLoading(false)
    })
  }
  return (
    <>
      <Button onClick={checkUpdate} loading={checkLoading}>
        检查更新
      </Button>

      <Modal
        visible={updaterVisible}
        title="更新"
        onCancel={() => setUpdaterVisible(false)}
        cancelButtonProps={{ type: 'default' }}
        okText="下载并安装"
        confirmLoading={downloadLoading}
        onOk={downloadAndInstall}
      >
        <Form>
          <Form.Item label="最新版本" field="version">
            <Typography.Text className="font-bold">{updateInfo.version}</Typography.Text>
          </Form.Item>
          <Form.Item label="当前版本" field="current_version">
            <Typography.Text>{updateInfo.current_version}</Typography.Text>
          </Form.Item>
        </Form>

        {downloadLoading && <Progress percent={updaterProgress} size="large" style={{}} />}
      </Modal>
    </>
  )
}
