
export default ({ value, regEx, groups, className }) => {
  const formatted = value.replace(regEx, groups);

  return (
    <span className={className}>{formatted}</span>
  )
}