import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Percentage from "../components/percentage";
import * as Breadcrumb from "../components/breadcrumb";
import { useDataTable } from "../hooks/data";
import { App, getEpochMetaPath, getEpoch, getMeta, EpochView, getEpochRangePaths } from "../pkg";
import { createColumnHelper } from "@tanstack/react-table";
import Link from 'next/link';
import RelativeDatetime from "../components/relative-datetime";
import Badge from "../components/badge";
import { ClockCountdown } from "@phosphor-icons/react";


export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const metaPath = getEpochMetaPath(app);
  const meta = await fetch(metaPath)
    .then(r => r.blob())
    .then(b => b.arrayBuffer())
    .then(a => getMeta(a));
  return {
    props: {
      epochs: [],//await getEpochs(app, 0, 10, "default", false, meta.count),
      epochsCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const columnHelper = createColumnHelper<EpochView>()

  const columns = [
    columnHelper.accessor("epoch", {
      header: "Epoch",
      cell: props => <Link href={`/epoch/${props.getValue()}`}>
        {props.getValue()}
      </Link>
    }),
    columnHelper.accessor("timestamp", {
      header: "Time",
      cell: props => <><RelativeDatetime timestamp={props.getValue()} /> ago</>

    }),
    columnHelper.accessor(
      row => ({ p: row.proposedBlocksCount, m: row.missedBlocksCount, o: row.orphanedBlocksCount }),
      {
        header: "Blocks",
        cell: props =>
          <>
            <Badge className="text-sm highlight-green">
              <Number value={props.getValue().p} />
            </Badge>
            &nbsp;
            <Badge className="text-sm highlight-amber">
              <Number value={props.getValue().m} />
            </Badge>
            &nbsp;
            <Badge className="text-sm highlight-slate">
              <Number value={props.getValue().o} />
            </Badge>
          </>
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
      row => ({ p: row.proposerSlashingsCount, a: row.attesterSlashingsCount }),
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
    columnHelper.accessor("eligibleEther", {
      header: "Eligible",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("votedEther", {
      header: "Voted",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("globalParticipationRate", {
      header: "Rate",
      cell: props => <Percentage value={props.getValue()} />
    })
  ];

  const table = useDataTable(app, "epochs", { kind: "integers" }, getEpoch, getEpochRangePaths, columns, props.epochsCount);

  return (
    <>
      <Breadcrumb.Root>
        <Breadcrumb.Part>
          <>
            <ClockCountdown />&nbsp;Epochs
          </>
        </Breadcrumb.Part>
      </Breadcrumb.Root>
      <section>
        <DataTable table={table} />
      </section>
    </>
  )
}
