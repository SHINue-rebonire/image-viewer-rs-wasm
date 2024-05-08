import React from 'react'
import { Box } from '@mui/material'

type PreviewsProps = {
  image: string | ArrayBuffer | null
}

export const Previews: React.FC<PreviewsProps> = ({ image }) => {
  if (!image) return null

  return (
    <Box
      sx={{
        display: 'flex',
        textAlign: 'center',
        width: '100%',
        height: '80vh',
        overflow: 'hidden',
      }}
    >
      <img
        src={image as string}
        style={{
          display: 'block',
          margin: 'auto',
          maxWidth: '100%',
          maxHeight: '100%',
          objectFit: 'contain',
        }}
        alt="Uploaded"
      />
    </Box>
  )
}
