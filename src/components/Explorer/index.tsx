import { Editors, Fs } from '@utils'
import type { FC } from 'react'
import { useState } from 'react'
import { useEditorStore } from '@stores'
import { APP_NAME } from '@constants'
import type { FileEntry } from '@tauri-apps/api/fs'
import { readDir, readTextFile } from '@tauri-apps/api/fs'
import { open } from '@tauri-apps/api/dialog'
import { FileTree, Icon } from '@components'
import { appWindow } from '@tauri-apps/api/window'
import classNames from 'classnames'

const Explorer: FC<ExplorerProps> = (props) => {
  const { editors, folderData, setFolderData } = useEditorStore()
  const [selectedPath, setSelectedPath] = useState<string>()

  const handleSelect = async (item: FileEntry) => {
    if (item.children)
      return

    setSelectedPath(item?.path)
    appWindow.setTitle(item?.name || APP_NAME)
    const text = await readTextFile(item.path)
    Editors.setMarkDown(text)
  }

  const handleOpenFileClick = async () => {
    const res = await Fs.selectMdFileAndRead()
    if (res !== undefined)
      Editors.setMarkDown(res.content)
  }

  const handleOpenDirClick = async () => {
    const dir = await open({ directory: true, recursive: true })

    if (!dir)
      return
    try {
      const res = await readDir(dir, { recursive: true })
      setFolderData(res)
    }
    catch (error) {
      console.log('error', error)
    }
  }

  const containerCLs = classNames('w-full flex flex-col', props.className)

  return (
    <div className={containerCLs}>
      <div className="border-b-1 flex justify-between items-center px-4 py-1">
        <small>EXPLORER</small>
        <div className="flex">
        </div>
      </div>
      <FileTree className="flex-1" data={folderData} selectedPath={selectedPath} onSelect={handleSelect}></FileTree>
      <div className="border-t-1 flex justify-between items-center px-4 py-1" >
        <small className="flex-1" onClick={handleOpenDirClick}>open dir</small>
        <Icon name="moreVertical" iconProps={{ className: 'w-20px h-20px icon-hover cursor-pointer' }}/>
      </div>
    </div>
  )
}

interface ExplorerProps {
  className?: string
}

export default Explorer
