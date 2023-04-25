import { BigNumber } from "bignumber.js";

type Props = { className?: string, value: number };


export default ({ value, className }: Props) => {
  const formatted = new BigNumber(value).toFormat();

  return (
    <span className={className}>{formatted}</span>
  )
}