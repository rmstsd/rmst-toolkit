import React from 'react'
import ReactDOM from 'react-dom/client'
import { ConfigProvider } from '@arco-design/web-react'
import App from './App'

import '@arco-design/web-react/dist/css/arco.css'
import './main.less'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <ConfigProvider componentConfig={{ Button: { type: 'primary' } }}>
    <App />
  </ConfigProvider>
)
