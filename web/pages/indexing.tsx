import { createColumnHelper } from "@tanstack/react-table";
import Link from "next/link";
import DataTable from "../components/data-table";
import Root from "../components/root";
import useDataTable from "../hooks/data-table";
import { App, BlockRequestView, getBlockRequest, getBlockRequestMeta } from "../pkg/web";


export async function getStaticProps() {
  const host = process.env.HOST;
  const app = new App("http://localhost:3000");
  const meta = await getBlockRequestMeta(app);
  return {
    props: {
      host,
      blockRequests: [],//await getEpochs(app, 0, 10, "default", false, meta.count),
      blockRequestsCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(props.host);
  const columnHelper = createColumnHelper<BlockRequestView>();

  const columns = [
    columnHelper.accessor("root", {
      header: "Root",
      cell: props =>
        <Root className="font-mono" value={props.getValue()} />
    }),
    columnHelper.accessor("state", { header: "State" }),
    columnHelper.accessor("activeRequestCount", { header: "Active requests count" }),
    columnHelper.accessor("failedCount", { header: "Failed count" }),
    columnHelper.accessor("notFoundCount", { header: "Not found count" }),
    columnHelper.accessor("foundBy", {
      header: "Found by",
      cell: props => <Root value={props.getValue()} />
    }),
  ]

  const table = useDataTable(app, "block_requests", getBlockRequest, columns, props.blockRequestsCount, "root");

  return (
    <>
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable table={table} />
        </div>
      </section>
    </>
  )

}