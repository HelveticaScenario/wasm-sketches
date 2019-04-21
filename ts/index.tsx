import 'pepjs'

import React from 'react'
import ReactDOM from 'react-dom'
import { BrowserRouter as Router, Route, Link } from 'react-router-dom'
import { WasmContext, WasmModule, SketchDescription } from './wasmContext'
import App from './app'
import { uw } from './utils'

function getSketches(module: WasmModule): ReadonlyArray<SketchDescription> {
	const count = module.get_sketch_count()
	const sketches: Array<SketchDescription> = []
	for (let i = 0; i < count; i++) {
		sketches[i] = {
			index: i,
			name: module.get_sketch_name(i),
			url: module.get_sketch_url(i),
			isDesktop: uw(module.get_sketch_is_desktop(i)),
			isMobile: uw(module.get_sketch_is_mobile(i)),
			isPublic: uw(module.get_sketch_is_public(i)),
		}
	}
	return sketches
}

import('../crate/pkg/rust_webpack').then(module => {
	const sketches = getSketches(module)
	const context = {
		module,
		sketches,
	}
	const Root = (props: {}) => (
		<WasmContext.Provider value={context}>
			<Router>
				<App />
			</Router>
		</WasmContext.Provider>
	)

	ReactDOM.render(<Root />, document.getElementById('app'))
})

// import('../crate/pkg/rust_webpack').then(module => {
// 	const sketches = getSketches(module)
// 	console.log(sketches)
// 	window.addEventListener('click', event => {
// 		if (event.target instanceof HTMLAnchorElement) {
// 			event.preventDefault()
// 			event.stopImmediatePropagation()
// 			const {
// 				href,
// 				textContent,
// 				dataset: { index },
// 			} = event.target
// 			const indexInt = parseInt(index)
// 			if (!Number.isNaN(indexInt)) {
// 				history.pushState({ index: parseInt(index) }, textContent, href)
// 				runsSketch(sketches[indexInt], module)
// 			}
// 		}
// 	})
// 	window.addEventListener('popstate', e => {
// 		debugger
// 		console.log(e.state)
// 	})
// 	if (location.pathname === '/') {
// 		loadDirectory(sketches)
// 		return
// 	}
// 	const pathRegex = /^\/(.*)\/?$/
// 	const pathname = pathRegex.exec(location.pathname)
// 	console.log(pathname[1])
// 	const sketch = sketches.find(({ url }) => url === pathname[1])
// 	console.log(sketch)
// 	if (sketch == null) {
// 		console.log(sketch)
// 		document.body.textContent = 'Sketch Not Found'
// 		return
// 	}
// 	runSketch(sketch, module)
// })
