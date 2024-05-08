import React, { useState } from 'react'
import { Box, Button, AppBar, Toolbar, Typography } from '@mui/material'
import { Image } from 'image-js'
import { Previews } from './components/Previews/Previews'
import * as flatbuffers from 'flatbuffers'
import { ImageProcessing } from './fbs/image-processing'
import init, { process_image } from '../rust_wasm/pkg'

init()

const App = () => {
  const [imageUrl, setImageUrl] = useState<string | ArrayBuffer | null>(null)
  const [fbBuffer, setFbBuffer] = useState<Uint8Array | null>(null)

  const handleFileChange = async (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    const file = event.target.files?.[0]
    if (!file) {
      setImageUrl(null)
      setFbBuffer(null)
      return
    }

    try {
      const image = await Image.load(await file.arrayBuffer())
      setImageUrl(image.toDataURL())

      // FlatBuffersのビルダーを準備
      const builder = new flatbuffers.Builder(1024)

      // 画像データ（ピクセル値）をUInt8Arrayとして取得
      const imageData = new Uint8Array(image.getRGBAData())

      // FlatBuffersのバッファに画像データを格納
      const imageDataOffset = ImageProcessing.createBufVector(
        builder,
        imageData,
      )

      ImageProcessing.startImageProcessing(builder)
      ImageProcessing.addWidth(builder, image.width)
      ImageProcessing.addHeight(builder, image.height)
      ImageProcessing.addBuf(builder, imageDataOffset)
      const imageProc = ImageProcessing.endImageProcessing(builder)
      builder.finish(imageProc)

      // Uint8ArrayとしてエンコードされたFlatBuffersバイナリを取得
      const fbBuffer = builder.asUint8Array()
      setFbBuffer(fbBuffer)
    } catch (error) {
      setImageUrl(null)
      setFbBuffer(null)
    }
  }

  const handleProcessImage = async () => {
    if (!fbBuffer) {
      return
    }
    try {
      const processedBuffer = process_image(fbBuffer)
      const buf = new flatbuffers.ByteBuffer(processedBuffer)
      const imageProc = ImageProcessing.getRootAsImageProcessing(buf)

      const width = imageProc.width()
      const height = imageProc.height()
      const pixels = imageProc.bufArray() as Uint8Array

      const img = new Image(width, height, pixels, {
        components: 1,
        alpha: 0,
        bitDepth: 8,
      })
      const imgFile = img.toBuffer({ format: 'png' })
      const blob = new Blob([imgFile.buffer], { type: 'image/png' })
      const imgUrl = URL.createObjectURL(blob)
      setImageUrl(imgUrl)
    } catch (error) {
      console.error(`Error creating image from buffer: `, error)
    }
  }

  return (
    <div>
      <AppBar position="fixed">
        <Toolbar>
          <Typography variant="h6" component="div" sx={{ flexGrow: 1 }}>
            Image Processor
          </Typography>
          <input
            accept="image/png"
            style={{ display: 'none' }}
            id="raised-button-file"
            multiple
            type="file"
            onChange={handleFileChange}
          />
          <label htmlFor="raised-button-file">
            <Button component="span" color="inherit">
              Load Image
            </Button>
          </label>
          <Button color="inherit" onClick={handleProcessImage}>
            Process Image
          </Button>
        </Toolbar>
      </AppBar>
      <Box
        sx={{
          pt: 12,
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          width: '100%',
          height: 'calc(100vh - 64px)',
        }}
      >
        <Previews image={imageUrl} />
      </Box>
    </div>
  )
}

export default App
