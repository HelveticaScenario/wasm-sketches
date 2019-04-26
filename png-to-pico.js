const { PNG } = require('pngjs')
const fs = require('fs')
const path = require('path')
const [, , file] = process.argv

if (file) {
	const { width, height, palette, data, colorType } = PNG.sync.read(
		fs.readFileSync(file)
	)
	if (colorType === 3) {
		const spritesheet = Buffer.alloc(width * height)
		console.log(width, height)
		const paletteBuffer = Buffer.alloc(256 * 4, 0)
		const paletteSearchBuffer = Buffer.alloc(palette.length)
		for (let i = 0; i < palette.length; i++) {
			paletteBuffer[i * 4] = palette[i][0]
			paletteBuffer[i * 4 + 1] = palette[i][1]
			paletteBuffer[i * 4 + 2] = palette[i][2]
			paletteBuffer[i * 4 + 3] = palette[i][3]
		}
		for (let i = 0; i < width * height; i++) {
			let offset = i * 4
			const index = paletteBuffer.indexOf(data.slice(offset, offset + 4)) / 4
			spritesheet[i] = Math.min(255, Math.max(0, index))
		}
		const sizes = Buffer.alloc(4)
		sizes.writeUInt16LE(width, 0)
		sizes.writeUInt16LE(height, 2)
		const realPaletteBuffer = Buffer.alloc(256 * 3, 0)
		for (let i = 0; i < 256; i++) {
			realPaletteBuffer[i * 3] = paletteBuffer[i * 4]
			realPaletteBuffer[i * 3 + 1] = paletteBuffer[i * 4 + 1]
			realPaletteBuffer[i * 3 + 2] = paletteBuffer[i * 4 + 2]
		}
		const ret = Buffer.concat([sizes, realPaletteBuffer, spritesheet])

		fs.writeFileSync(
			path.resolve(file, '..', path.basename(file, '.png') + '.pico'),
			ret
		)
	}
}
