;(() => {
  window.addEventListener('load', () => {
    const icon = getFaviconUrl()
    send(document.title, icon)

    const iconElements = document.querySelectorAll('link[rel*="icon"]')
    if (iconElements.length > 0) {
      icon = iconElements[0].href
    }

    // 获取 title 元素
    const titleElement = document.querySelector('title')

    // 如果没有找到 title 元素，则退出
    if (!titleElement) {
      console.warn('未找到 title 元素')
      return
    }

    // 创建一个 MutationObserver 实例
    const observer = new MutationObserver(mutationsList => {
      for (const mutation of mutationsList) {
        if (mutation.type === 'childList') {
          send(document.title, icon)
        }
      }
    })

    // 配置观察选项
    const config = { childList: true }
    // 开始观察 title 元素
    observer.observe(titleElement, config)
  })
})()

function send(title, icon) {
  window.__TAURI_INTERNALS__.invoke('page_loaded', { title, icon })
}

function getFaviconUrl() {
  // 查找所有 link 标签
  const links = document.querySelectorAll('link')

  // 定义可能的 rel 属性值优先级
  const relPriorities = ['icon', 'shortcut icon', 'apple-touch-icon', 'apple-touch-icon-precomposed']

  // 按优先级查找匹配的 icon
  for (const rel of relPriorities) {
    const icon = Array.from(links).find(link => link.rel.toLowerCase() === rel && link.href)

    if (icon) {
      return new URL(icon.href, document.baseURI).href
    }
  }

  // 如果没有找到特定 rel 的 icon，尝试查找任意 rel 包含 "icon" 的标签
  const anyIcon = Array.from(links).find(link => link.rel.toLowerCase().includes('icon') && link.href)

  if (anyIcon) {
    return new URL(anyIcon.href, document.baseURI).href
  }

  // 如果没有找到任何 icon 标签，返回默认路径
  return new URL('/favicon.ico', document.baseURI).href
}
