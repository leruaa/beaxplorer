
import { App, getBlockMeta } from "../pkg";
import Breadcrumb from "../components/breadcrumb";
import BlocksTable from "../components/blocks/blocks-table";

export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const meta = await getBlockMeta(app);
  return {
    props: {
      blocks: [], //await blocks.page(pageIndex || 0, 10, "default", false),
      blocksCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Blocks", icon: "cube" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <BlocksTable app={app} blocksCount={props.blocksCount} kind={{ kind: "integers" }} />
        </div>
      </section>
    </>
  )
}
