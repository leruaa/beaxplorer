
import DataTable from "../components/data-table";
import Number from "../components/number";
import Breadcrumb from "../components/breadcrumb";
import { App, BlockView, getBlock, getBlockMeta } from "../pkg";
import { createColumnHelper } from "@tanstack/react-table";
import useDataTable from "../hooks/data-table";
import Badge from "../components/badge";

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
  const columnHelper = createColumnHelper<BlockView>()

  const columns = [
    columnHelper.accessor("epoch", {
      header: "Epoch",
      cell: props => <a href={`/block/${props.getValue()}`}><Number value={props.getValue()} /></a>
    }),
    columnHelper.accessor("slot", {
      header: "Block",
      cell: props => <a href={`/block/${props.getValue()}`}><Number value={props.getValue()} /></a>
    }),
    columnHelper.accessor("status", {
      header: "Status",
      cell: props => {
        switch (props.getValue()) {
          case "Proposed":
            return (
              <Badge className="bg-green-50 text-green-500">Proposed</Badge>
            )
          case "Missed":
            return (
              <Badge className="bg-amber-50 text-amber-500">Missed</Badge>
            )
          case "Orphaned":
            return (
              <Badge className="bg-slate-50 text-slate-500">Orphaned</Badge>
            )
        }
      }
    }),
    columnHelper.accessor("proposer", {
      header: "Proposer"
    }),
    columnHelper.accessor("attestationsCount", {
      header: "Attestations",
      cell: props => <Number value={props.getValue()} />
    }),
    columnHelper.accessor("depositsCount", {
      header: "Deposits",
      cell: props => <Number value={props.getValue()} />
    }),
    columnHelper.accessor(
      (row, rowIndex) => { return { p: row.proposerSlashingsCount, a: row.attesterSlashingsCount } },
      {
        header: "Slashings P / A",
        cell: props => <>
          <Number value={props.getValue().p} /> / <Number value={props.getValue().a} />
        </>
      }
    ),
    columnHelper.accessor("voluntaryExitsCount", {
      header: "Exits",
      cell: props => <Number value={props.getValue()} />
    })
  ];

  const table = useDataTable(app, "blocks", { kind: "integers" }, getBlock, columns, props.blocksCount);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Blocks", icon: "cube" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable table={table} />
        </div>
      </section>
    </>
  )
}
