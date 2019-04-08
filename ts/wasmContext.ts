import React from 'react'
import { uw } from './utils'

export type WasmModule = typeof import('../crate/pkg/rust_webpack')
export interface SketchDescription {
	index: number
	name: string
	url: string
	isDesktop: boolean
	isMobile: boolean
	isPublic: boolean
}
interface ContextValue {
	module: WasmModule
	sketches: ReadonlyArray<SketchDescription>
}
export const unwrapContextValue = (
	ctx: Partial<ContextValue>
): ContextValue => {
	return {
		module: uw(ctx.module),
		sketches: uw(ctx.sketches),
	}
}
export const WasmContext = React.createContext<Partial<ContextValue>>({})
