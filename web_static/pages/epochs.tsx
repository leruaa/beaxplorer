import Breadcrumb from "../components/breadcrumb";

export default () => {
    return (
      <>
        <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
        <section className="container mx-auto">
            <div className="tabular-data">
            <p>Showing epochs</p>
            <div id="epochs-list"></div>
            </div>
        </section>
      </>
    )
  }
  