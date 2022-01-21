import { BigNumber } from "bignumber.js";

export default ({ value }) => {
  const formatted = new BigNumber(value).multipliedBy(100).toFormat(2);

  return (
    <span>{formatted}%</span>
  )
}