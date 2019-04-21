import { uw } from './utils'
import { SketchDescription, WasmModule } from './wasmContext'

function getQueryVariable(variable: string) {
	var query = window.location.search.substring(1)
	var vars = query.split('&')
	for (var i = 0; i < vars.length; i++) {
		var pair = vars[i].split('=')
		if (decodeURIComponent(pair[0]) == variable) {
			return decodeURIComponent(pair[1])
		}
	}
	console.log('Query variable %s not found', variable)
}

const linearMode = getQueryVariable('linear') === 'true'
const thirty = getQueryVariable('thirty') === 'true'

function createShader(gl: WebGLRenderingContext, source: string, type: number) {
	const shader = uw(gl.createShader(type))
	gl.shaderSource(shader, source)
	gl.compileShader(shader)
	if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		return shader
	}
	let msg = uw(gl.getShaderInfoLog(shader))
	alert(msg)
	throw new Error(msg)
}

function createTexture(gl: WebGLRenderingContext) {
	const texture = gl.createTexture()
	gl.bindTexture(gl.TEXTURE_2D, texture)
	gl.texParameteri(
		gl.TEXTURE_2D,
		gl.TEXTURE_MIN_FILTER,
		linearMode ? gl.LINEAR : gl.NEAREST
	)
	gl.texParameteri(
		gl.TEXTURE_2D,
		gl.TEXTURE_MAG_FILTER,
		linearMode ? gl.LINEAR : gl.NEAREST
	)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
	return texture
}

export default (
	gl: WebGLRenderingContext,
	canvas: HTMLCanvasElement,
	sketch: SketchDescription,
	module: WasmModule
) => {
	const memory: WebAssembly.Memory = module.get_memory()
	const { index } = sketch
	module.init(index)

	const screenSize = module.screen_size()
	const paletteSize = module.palette_size()
	const swapSize = module.palette_swap_size()
	const screenWidth = module.screen_width()
	const screenHeight = module.screen_height()
	let viewportWidth: number
	let viewportHeight: number
	let dim: number
	const resize = () => {
		// dim = Math.min(
		// 	Math.floor(window.innerWidth / screenWidth),
		// 	Math.floor(window.innerHeight / screenHeight)
		// )
		// if (dim < 1) {
		dim = Math.min(
			window.innerWidth / screenWidth,
			window.innerHeight / screenHeight
		)
		// }
		const width = Math.floor(screenHeight * dim)
		const height = Math.floor(screenWidth * dim)
		canvas.style.height = `${width}px`
		canvas.style.width = `${height}px`
		viewportHeight = height * devicePixelRatio
		viewportWidth = width * devicePixelRatio
		canvas.height = viewportHeight
		canvas.width = viewportWidth
	}

	function getPointerPos(evt: PointerEvent) {
		var rect = canvas.getBoundingClientRect()
		return {
			x: Math.floor((evt.clientX - rect.left) / dim),
			y: Math.floor((evt.clientY - rect.top) / dim),
		}
	}

	function getMousePos(evt: MouseEvent) {
		var rect = canvas.getBoundingClientRect()
		return {
			x: Math.floor((evt.clientX - rect.left) / dim),
			y: Math.floor((evt.clientY - rect.top) / dim),
		}
	}

	function getTouchPos(touch: Touch) {
		var rect = canvas.getBoundingClientRect()
		return {
			x: Math.floor((touch.clientX - rect.left) / dim),
			y: Math.floor((touch.clientY - rect.top) / dim),
		}
	}

	canvas.addEventListener('contextmenu', e => {
		e.preventDefault()
		e.stopImmediatePropagation()
		return false
	})

	window.addEventListener('resize', resize)
	resize()

	canvas.addEventListener('wheel', e => {
		e.preventDefault()
		e.stopPropagation()
		let delta = e.deltaY
		if (e.ctrlKey) {
			delta *= -1
		}
		module.set_wheel(delta)
	})

	const touchState = Array(10)
	touchState.fill(null)

	canvas.addEventListener('pointerenter', e => {
		const { pointerId } = e
		const index = touchState.indexOf(null)
		if (index >= 0) {
			touchState[index] = pointerId
			const { x, y } = getPointerPos(e)
			module.set_pointer_pos(index, x, y)
		}
	})

	canvas.addEventListener('pointerout', e => {
		const { pointerId } = e
		const index = touchState.indexOf(pointerId)
		if (index >= 0) {
			touchState[index] = null
			module.set_pointer_end(index)
		}
	})

	canvas.addEventListener('pointerdown', e => {
		const { pointerId } = e
		const index = touchState.indexOf(pointerId)
		if (index >= 0) {
			module.set_pointer_state(index, e.buttons)
		}
	})

	canvas.addEventListener('pointerup', e => {
		const { pointerId } = e
		const index = touchState.indexOf(pointerId)
		if (index >= 0) {
			module.set_pointer_state(index, e.buttons)
		}
	})

	canvas.addEventListener('pointermove', e => {
		const { pointerId } = e
		const index = touchState.indexOf(pointerId)
		if (index >= 0) {
			const { x, y } = getPointerPos(e)
			module.set_pointer_pos(index, x, y)
		}
	})

	const vertShaderSource = `
  precision mediump float;
  precision mediump int;

  attribute vec2 a_position;
  attribute vec2 a_texCoord;

  varying vec2 v_texCoord;
  void main() {
      v_texCoord = a_texCoord;
      gl_Position = vec4(a_position,0, 1);
  }`

	const fragShaderSource = `
  precision mediump float;
  precision mediump int;

	uniform sampler2D u_palette;     //256 x 1 pixels
	uniform sampler2D u_swap;        //256 x 1 pixels
  uniform sampler2D u_screen;
  varying vec2 v_texCoord;

  void main()
  {
      gl_FragColor = texture2D(
				u_palette,
				texture2D(
					u_swap,
					texture2D(
						u_screen,
						v_texCoord
					).xy
				).xy
			);
  }
  `

	const vertShader = createShader(gl, vertShaderSource, gl.VERTEX_SHADER)
	const fragShader = createShader(gl, fragShaderSource, gl.FRAGMENT_SHADER)
	const program = uw(gl.createProgram())
	gl.attachShader(program, vertShader)
	gl.attachShader(program, fragShader)
	gl.linkProgram(program)
	if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
		const info = gl.getProgramInfoLog(program)
		throw 'Could not compile WebGL program. \n\n' + info
	}
	gl.useProgram(program)

	const vertexPositionAttribute = gl.getAttribLocation(program, 'a_position')
	const textureCoordsAttribute = gl.getAttribLocation(program, 'a_texCoord')
	const paletteUniform = gl.getUniformLocation(program, 'u_palette')
	const swapUniform = gl.getUniformLocation(program, 'u_swap')
	const screenUniform = gl.getUniformLocation(program, 'u_screen')
	gl.enableVertexAttribArray(vertexPositionAttribute)
	gl.enableVertexAttribArray(textureCoordsAttribute)
	const paletteTexture = createTexture(gl)
	const swapTexture = createTexture(gl)
	const screenTexture = createTexture(gl)
	const vertBuffer = gl.createBuffer()
	gl.bindBuffer(gl.ARRAY_BUFFER, vertBuffer)
	gl.bufferData(
		gl.ARRAY_BUFFER,
		new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
		gl.STATIC_DRAW
	)
	var uvBuffer = gl.createBuffer()
	gl.bindBuffer(gl.ARRAY_BUFFER, uvBuffer)
	gl.bufferData(
		gl.ARRAY_BUFFER,
		new Float32Array([0, 1, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0]),
		gl.STATIC_DRAW
	)
	gl.clearColor(0, 0, 0, 1)
	gl.enable(gl.DEPTH_TEST)
	const screen = new Uint8Array(memory.buffer, module.screen_ptr(), screenSize)
	const swap = new Uint8Array(
		memory.buffer,
		module.palette_swap_ptr(),
		swapSize
	)
	const palette = new Uint8Array(
		memory.buffer,
		module.palette_ptr(),
		paletteSize
	)
	function updateTextures() {
		gl.bindTexture(gl.TEXTURE_2D, screenTexture)
		gl.texImage2D(
			gl.TEXTURE_2D,
			0,
			gl.LUMINANCE,
			screenWidth,
			screenHeight,
			0,
			gl.LUMINANCE,
			gl.UNSIGNED_BYTE,
			screen
		)
		gl.bindTexture(gl.TEXTURE_2D, swapTexture)
		gl.texImage2D(
			gl.TEXTURE_2D,
			0,
			gl.LUMINANCE,
			swapSize,
			1,
			0,
			gl.LUMINANCE,
			gl.UNSIGNED_BYTE,
			swap
		)
		gl.bindTexture(gl.TEXTURE_2D, paletteTexture)
		gl.texImage2D(
			gl.TEXTURE_2D,
			0,
			gl.RGB,
			256,
			1,
			0,
			gl.RGB,
			gl.UNSIGNED_BYTE,
			palette
		)
	}

	let count = 0
	let last: number
	let toggle = false
	return (now: number) => {
		// if (!canvas.parentNode) {
		// 	console.log('removed')
		// 	return
		// }

		count++
		count = count % 16
		// window.requestAnimationFrame(run)
		toggle = !toggle
		if (thirty && !toggle) {
			return
		}
		if (last == null) {
			last = now
		}
		module.update(now - last)
		last = now

		gl.viewport(0, 0, viewportWidth, viewportHeight)
		gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)
		updateTextures()
		gl.bindBuffer(gl.ARRAY_BUFFER, vertBuffer)
		gl.vertexAttribPointer(vertexPositionAttribute, 2, gl.FLOAT, false, 0, 0)
		gl.bindBuffer(gl.ARRAY_BUFFER, uvBuffer)
		gl.vertexAttribPointer(textureCoordsAttribute, 2, gl.FLOAT, false, 0, 0)

		gl.activeTexture(gl.TEXTURE0)
		gl.bindTexture(gl.TEXTURE_2D, screenTexture)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MIN_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MAG_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
		gl.uniform1i(screenUniform, 0)

		gl.activeTexture(gl.TEXTURE1)
		gl.bindTexture(gl.TEXTURE_2D, swapTexture)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MIN_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MAG_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
		gl.uniform1i(swapUniform, 1)

		gl.activeTexture(gl.TEXTURE2)
		gl.bindTexture(gl.TEXTURE_2D, paletteTexture)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MIN_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(
			gl.TEXTURE_2D,
			gl.TEXTURE_MAG_FILTER,
			linearMode ? gl.LINEAR : gl.NEAREST
		)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
		gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)
		gl.uniform1i(paletteUniform, 2)

		gl.drawArrays(gl.TRIANGLES, 0, 6)

		// console.log(screen[0])
	}
}
