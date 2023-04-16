
type Props = { bits: boolean[] };

export default ({ bits }: Props) => {
  let chunks = [];

  for (let i = 0; i < bits.length; i += 64) {
    const line = bits.slice(i, i + 64);

    for (let j = 0; j < line.length; j += 8) {
      const byte = line.slice(j, j + 8);

      chunks.push(
        <span>{byte.map(b => b ? "1" : "0").join("")}</span>
      )
    }
  }

  return <div className="font-mono grid grid-cols-8 gap-1">{chunks}</div>
}