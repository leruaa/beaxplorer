export default (props) => {
  if (props.id) {
    return (
      <svg className={`icon ${props.className || ""}`}>
        <use xlinkHref={`/svg/${props.v}.svg#${props.id}`}></use>
      </svg>
    )
  }

  return null;
}