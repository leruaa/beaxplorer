import Icon from "./icon";

const BreadcrumbPart = (props) => {
  if (props.link) {
    return (
      <>
        <Icon className="text-gray-500" v="solid" id="chevron-right" />
          <a href="{{link}}">
            <Icon v="outline" id={props.icon} />
            {props.text}
          </a>
      </>
    );
  }
  else {
    return (
      <>
        <Icon className="text-gray-500" v="solid" id="chevron-right" />
        <Icon v="outline" id={props.icon} />
        {props.text}
      </>
    );
  }
  
}

export default (props) => {
  if (props.breadcrumb) {
    const parts = props.breadcrumb.parts.map((value) => {
      return <BreadcrumbPart props={value} />
    });

    return (
      <ul className="breadcrumb">
        <a href="/"><Icon v="solid" id="home" /></a>
        {parts}
      </ul>
    );
  }

  return null;
}