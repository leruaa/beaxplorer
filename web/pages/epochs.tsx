import { useRouter } from 'next/router'
import moment from "moment";
import Moment from 'react-moment';
import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Percentage from "../components/percentage";
import Breadcrumb from "../components/breadcrumb";
import useDataTable from "../hooks/data-table";
import { App, EpochExtendedView, getEpochMeta, getEpoch } from "../pkg";
import { createColumnHelper } from "@tanstack/react-table";


export async function getStaticProps() {
  const host = process.env.HOST;
  const app = new App("http://localhost:3000");
  const meta = await getEpochMeta(app);
  return {
    props: {
      host,
      epochs: [],//await getEpochs(app, 0, 10, "default", false, meta.count),
      epochsCount: meta.count
    }
  }
}

export default (props) => {

  const router = useRouter()
  const { page } = router.query
  const app = new App(props.host);
  const columnHelper = createColumnHelper<EpochExtendedView>()

  const columns = [
    columnHelper.accessor("epoch", {
      header: "Epoch",
      cell: props => <a href={`/epoch/${props.getValue()}`}><Number value={props.getValue()} /></a>
    }),
    columnHelper.accessor("timestamp", {
      header: "Time",
      cell: props =>
        <span title={moment.unix(props.getValue()).format("L LTS")}>
          <Moment unix fromNow date={props.getValue()} />
        </span>
    }),
    columnHelper.accessor("attestations_count", {
      header: "Attestations",
      cell: props => <Number value={props.getValue()} />
    }),
    columnHelper.accessor("deposits_count", {
      header: "Deposits",
      cell: props => <Number value={props.getValue()} />
    }),
    columnHelper.accessor(
      (row, rowIndex) => ({ p: row.proposer_slashings_count, a: row.attester_slashings_count }),
      {
        header: "Slashings P / A",
        cell: props =>
          <>
            <Number value={props.getValue().p} /> / <Number value={props.getValue().a} />
          </>
      }),
    columnHelper.accessor("finalized", {
      header: "Finalized",
      cell: props => props.getValue() ? "Yes" : "No"
    }),
    columnHelper.accessor("eligible_ether", {
      header: "Eligible",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("voted_ether", {
      header: "Voted",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("global_participation_rate", {
      header: "Rate",
      cell: props => <Percentage value={props.getValue()} />
    })
  ];

  const table = useDataTable(app, "epochs", getEpoch, columns, props.epochsCount);

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <DataTable table={table} />
        </div>
      </section>
    </>
  )
}
