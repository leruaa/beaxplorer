const BreadcrumbPart = (props) => {
  if (props.link) {
    return (
      <>
        <i className="icon solid-chevron-right text-gray-500" />
        <a href="{{link}}">
          <i className={`icon outline-${props.icon}`} /> 
          &nbsp;
          {props.text}
        </a>
      </>
    );
  }
  else {
    return (
      <>
        <i className="icon solid-chevron-right text-gray-500" />
        <i className={`icon outline-${props.icon}`} />
        &nbsp;
        {props.text}
      </>
    );
  }
  
}

export default (props) => {
  if (props.breadcrumb) {
    const parts = props.breadcrumb.parts.map((value) => {
      return <BreadcrumbPart{...value} />
    });

    return (
      <h2 className="container mx-auto">
        <ul className="breadcrumb">
          <a href="/"><i className="icon solid-home" /></a>
          {parts}
        </ul>
      </h2>
    );
  }

  return null;
}