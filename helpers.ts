
export const formatAntList = (ants: number[]) => (
    ants.reduce((s: string, a: number, i: number) => {
        if (i === 0) {
            return `ant ${a}`
        } else {
            return (s + ` and ant ${a}`)
        }
    }, "")
)

