import { BigNumber } from "bignumber.js";

export default ({ value }) => {
  const formatted = new BigNumber(value).div(Math.pow(10, 9)).toFormat();

  return (
    <>{formatted}</>
  )
}