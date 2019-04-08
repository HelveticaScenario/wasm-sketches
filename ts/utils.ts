export const uw = <T>(val: T | null | undefined) => {
	if (val == null) {
		throw new Error('PANIC')
	}
	return val
}
