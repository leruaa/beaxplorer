import { createColumnHelper } from "@tanstack/react-table";
import Link from "next/link";
import DataTable from "../components/data-table";
import Root from "../components/root";
import Peer from "../components/peer";
import useDataTable from "../hooks/data-table";
import { App, BlockRequestView, getBlockRequest, getBlockRequestMeta, getGoodPeer, getGoodPeerMeta, GoodPeerView } from "../pkg/web";


export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const blockRequestsMeta = await getBlockRequestMeta(app);
  const goodPeersMeta = await getGoodPeerMeta(app);
  return {
    props: {
      blockRequests: [],//await getEpochs(app, 0, 10, "default", false, meta.count),
      blockRequestsCount: blockRequestsMeta.count,
      goodPeersCount: goodPeersMeta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const blockRequestsColumnHelper = createColumnHelper<BlockRequestView>();
  const goodPeersColumnHelper = createColumnHelper<GoodPeerView>();

  const blockRequestsColumns = [
    blockRequestsColumnHelper.accessor("root", {
      header: "Root",
      cell: props =>
        <Root value={props.getValue()} />
    }),
    blockRequestsColumnHelper.accessor("possibleSlots", {
      header: "Possible slots",
      cell: props => props.getValue()
        .map(s =>
          <Link href={`/block/${s}`}>
            {s}
          </Link>)
        .reduce((accu: [JSX.Element], elem) => {
          return accu === null ? [elem] : [...accu, ', ', elem]
        }, null)
    }),
    blockRequestsColumnHelper.accessor("state", { header: "State" }),
    //blockRequestsColumnHelper.accessor("activeRequestCount", { header: "Active requests count" }),
    blockRequestsColumnHelper.accessor("failedCount", { header: "Failed count" }),
    blockRequestsColumnHelper.accessor("notFoundCount", { header: "Not found count" }),
    blockRequestsColumnHelper.accessor("foundBy", {
      header: "Found by",
      cell: props => <Peer className="font-mono" value={props.getValue()} />
    }),
  ]

  const goodPeersColumns = [
    goodPeersColumnHelper.accessor("id", {
      header: "Id",
      cell: props =>
        <Peer className="font-mono" value={props.getValue()} />
    }),
    goodPeersColumnHelper.accessor("address", { header: "Address" }),
  ]

  const blockRequestsTable = useDataTable(app, "block_requests", { kind: "strings" }, getBlockRequest, blockRequestsColumns, props.blockRequestsCount, "root");
  const goodPeersTable = useDataTable(app, "good_peers", { kind: "strings" }, getGoodPeer, goodPeersColumns, props.goodPeersCount, "id");

  return (
    <>
      <section className="container mx-auto">
        <div className="tabular-data">
          <h2>Block requests</h2>
          <DataTable table={blockRequestsTable} />
          <h2>Good peers</h2>
          <DataTable table={goodPeersTable} />
        </div>
      </section>
    </>
  )

}