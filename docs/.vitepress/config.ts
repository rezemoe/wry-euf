import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'WRY Unity',
  description: 'Embed native WebViews in Unity games and applications.',
  lang: 'en-US',
  cleanUrls: true,
  lastUpdated: true,
  themeConfig: {
    siteTitle: 'WRY Unity',
    logo: '/logo.svg',
    nav: [
      { text: 'Guide', link: '/getting-started' },
      { text: 'Unity API', link: '/unity-component' },
      { text: 'Platforms', link: '/platforms' },
      { text: 'Reference', link: '/native-api' },
      { text: 'WRY upstream', link: 'https://github.com/tauri-apps/wry' }
    ],
    sidebar: {
      '/': [
        {
          text: 'Start Here',
          items: [
            { text: 'Overview', link: '/' },
            { text: 'Getting Started', link: '/getting-started' },
            { text: 'Unity Component', link: '/unity-component' },
            { text: 'JavaScript and IPC', link: '/javascript-ipc' }
          ]
        },
        {
          text: 'Platforms',
          items: [
            { text: 'Platform Overview', link: '/platforms' },
            { text: 'Windows', link: '/windows' },
            { text: 'Linux', link: '/linux' },
            { text: 'Android', link: '/android' },
            { text: 'iOS', link: '/ios' }
          ]
        },
        {
          text: 'Project Reference',
          items: [
            { text: 'Native API', link: '/native-api' },
            { text: 'Architecture', link: '/ARCHITECTURE' },
            { text: 'Building', link: '/BUILDING' },
            { text: 'Security', link: '/security' },
            { text: 'Troubleshooting', link: '/troubleshooting' },
            { text: 'Contributing', link: '/contributing' }
          ]
        }
      ]
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/tauri-apps/wry' }
    ],
    footer: {
      message: 'Built on Tauri WRY.',
      copyright: 'MIT or Apache-2.0'
    },
    search: {
      provider: 'local'
    },
    outline: 'deep',
  }
})
