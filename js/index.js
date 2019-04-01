function getQueryVariable(variable) {
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

import('../crate/pkg/rust_webpack_bg').then(module => {
	const regex = /^\/sketch(\d)\/?$/
	const match = regex.exec(location.pathname)
	if (match == null) {
		document.body.textContent = 'Sketch Not Found'
		return
	}
	const index = parseInt(match[1])
	if (isNaN(index) || index < 1) {
		document.body.textContent = 'Sketch Not Found'
		return
	}
	const canvas = document.getElementById('canvas')
	const gl = canvas.getContext('webgl')
	console.log('init')

	console.log(index)
	console.log(module.init(index - 1))

	const screenSize = module.screen_size()
	const paletteSize = module.palette_size()
	const screenWidth = module.screen_width()
	const screenHeight = module.screen_height()
	let viewportWidth
	let viewportHeight
	let dim
	const resize = () => {
		dim = Math.min(
			Math.floor(window.innerWidth / screenWidth),
			Math.floor(window.innerHeight / screenHeight)
		)
		if (dim < 1) {
			dim = Math.min(
				window.innerWidth / screenWidth,
				window.innerHeight / screenHeight
			)
		}
		const width = Math.floor(screenHeight * dim)
		const height = Math.floor(screenWidth * dim)
		canvas.style.height = `${width}px`
		canvas.style.width = `${height}px`
		viewportHeight = height * devicePixelRatio
		viewportWidth = width * devicePixelRatio
		canvas.height = viewportHeight
		canvas.width = viewportWidth
	}

	function getMousePos(evt) {
		var rect = canvas.getBoundingClientRect()
		return {
			x: Math.floor((evt.clientX - rect.left) / dim),
			y: Math.floor((evt.clientY - rect.top) / dim),
		}
	}

	function getTouchPos(evt) {
		var rect = canvas.getBoundingClientRect()
		return {
			x: Math.floor((evt.touches[0].clientX - rect.left) / dim),
			y: Math.floor((evt.touches[0].clientY - rect.top) / dim),
		}
	}

	window.addEventListener('resize', resize)
	resize()

	canvas.addEventListener('mouseleave', e => {
		module.set_mouse_end()
	})
	canvas.addEventListener('mousemove', e => {
		const { x, y } = getMousePos(e)
		module.set_mouse_pos(x, y)
	})
	canvas.addEventListener('mouseenter', e => {
		const { x, y } = getMousePos(e)
		module.set_mouse_pos(x, y)
	})

	canvas.addEventListener('touchend', e => {
		e.preventDefault()
		e.stopImmediatePropagation()
		module.set_mouse_end()
	})
	canvas.addEventListener('touchstart', e => {
		e.preventDefault()
		e.stopImmediatePropagation()
		const { x, y } = getTouchPos(e)
		module.set_mouse_pos(x, y)
	})
	canvas.addEventListener('touchmove', e => {
		e.preventDefault()
		e.stopImmediatePropagation()
		const { x, y } = getTouchPos(e)
		module.set_mouse_pos(x, y)
	})

	const fragShaderSource = `
  precision mediump float;
  precision mediump int;
  
  uniform sampler2D u_palette;     //256 x 1 pixels
  uniform sampler2D u_screen;
  varying vec2 v_texCoord;
  
  void main()
  {
      //What color do we want to index?
      vec4 index = texture2D(u_screen, v_texCoord);
      //Do a dependency texture read
      vec4 texel = texture2D(u_palette, index.xy);
      gl_FragColor = texel;   //Output the color
  }
  `

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

	const fragShader = createShader(gl, fragShaderSource, gl.FRAGMENT_SHADER)
	const vertShader = createShader(gl, vertShaderSource, gl.VERTEX_SHADER)
	const program = gl.createProgram()
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
	const screenUniform = gl.getUniformLocation(program, 'u_screen')
	gl.enableVertexAttribArray(vertexPositionAttribute)
	gl.enableVertexAttribArray(textureCoordsAttribute)
	const paletteTexture = createTexture(gl)
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
	const screen = new Uint8Array(
		module.memory.buffer,
		module.screen_ptr(),
		screenSize
	)
	const palette = new Uint8Array(
		module.memory.buffer,
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
	let last = performance.now()
	let toggle = false
	const run = now => {
		count++
		count = count % 16
		window.requestAnimationFrame(run)
		toggle = !toggle
		if (thirty && !toggle) {
			return
		}
		// console.log(Math.floor(now - last));
		module.update(Math.floor(now - last))
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
		gl.uniform1i(paletteUniform, 1)
		gl.drawArrays(gl.TRIANGLES, 0, 6)

		// console.log(screen[0])
	}
	run()
})

function createShader(gl, source, type) {
	const shader = gl.createShader(type)
	gl.shaderSource(shader, source)
	gl.compileShader(shader)
	if (gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		return shader
	}
	alert(gl.getShaderInfoLog(shader))
}
function createTexture(gl) {
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
