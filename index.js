import { app, BrowserWindow } from 'electron'

function createWindow() {
  const win = new BrowserWindow({
    width: 1080,
    height: 720,
    webPreferences: {
      nodeIntegration: true
    }
  })
  win.loadFile('src/interface/window.html')
}

app.whenReady().then(createWindow)