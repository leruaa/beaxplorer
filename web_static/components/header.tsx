import Nav from "./nav";
import Breadcrumb from "./breadcrumb";

export default () => {
  return (
    <>
      <header>
        <Nav />
      </header>
      <h2 className="container mx-auto">
        <Breadcrumb />
      </h2>
    </>
  )
}