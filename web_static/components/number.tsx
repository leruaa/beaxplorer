import { BigNumber } from "bignumber.js";

export default ({ value }) => {
  const formatted = new BigNumber(value).toFormat();

  return (
    <span>{formatted}</span>
  )
}