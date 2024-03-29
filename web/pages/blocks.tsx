
import { App, getBlockMetaPath, getMeta } from "../pkg";
import * as Breadcrumb from "../components/breadcrumb";
import BlocksTable from "../components/blocks/blocks-table";
import { Cube } from "@phosphor-icons/react";
import { Accent, AccentContext } from "../hooks/accent";

export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const metaPath = getBlockMetaPath(app);
  const meta = await fetch(metaPath)
    .then(r => r.blob())
    .then(b => b.arrayBuffer())
    .then(a => getMeta(a));
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
    <AccentContext.Provider value={Accent.Indigo}>
      <Breadcrumb.Root>
        <Breadcrumb.Text>
          <Cube />&nbsp;Blocks
        </Breadcrumb.Text>
      </Breadcrumb.Root>
      <section>
        <div className="tabular-data">
          <BlocksTable app={app} blocksCount={props.blocksCount} kind={{ kind: "integers" }} />
        </div>
      </section>
    </AccentContext.Provider>
  )
}
