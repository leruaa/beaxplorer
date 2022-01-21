
export default ({ value, regEx, groups }) => {
  const formatted = value.replace(regEx, groups);

  return (
    <span>{formatted}</span>
  )
}