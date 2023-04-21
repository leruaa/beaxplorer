
type TrimProps = { value: string, regEx: RegExp, groups: string, className?: string };

export default ({ value, regEx, groups, className }: TrimProps) => {
  const formatted = value.replace(regEx, groups);

  return (
    <span className={className}>{formatted}</span>
  )
}