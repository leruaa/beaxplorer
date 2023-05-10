import { createColumnHelper } from "@tanstack/react-table";
import { useDataTable } from "../../hooks/data";
import { App, BlockView, RangeKind, getBlock, getBlockRangePaths } from "../../pkg/web";
import DataTable from "../data-table";
import Badge from "../badge";
import Number from "../number";

type Props = { app: App, blocksCount: number, kind: RangeKind };

export default ({ app, blocksCount, kind }: Props) => {
  const columnHelper = createColumnHelper<BlockView>()

  let columns = [];

  if (kind.kind != "epoch") {
    columns.push(
      columnHelper.accessor("epoch", {
        header: "Epoch",
        cell: props => <a href={`/block/${props.getValue()}`}><Number value={props.getValue()} /></a>,
        meta: { className: "text-right" }
      })
    );
  }

  columns = [
    ...columns,
    columnHelper.accessor("slot", {
      header: "Block",
      cell: props => <a href={`/block/${props.getValue()}`}><Number value={props.getValue()} /></a>,
      meta: { className: "text-right" }
    }),
    columnHelper.accessor("status", {
      header: "Status",
      cell: props => {
        switch (props.getValue()) {
          case "Proposed":
            return (
              <Badge className="text-sm highlight-green">Proposed</Badge>
            )
          case "Missed":
            return (
              <Badge className="text-sm highlight-amber">Missed</Badge>
            )
          case "Orphaned":
            return (
              <Badge className="text-sm highlight-slate">Orphaned</Badge>
            )
        }
      },
      enableSorting: false
    }),
    columnHelper.accessor("proposer", {
      header: "Proposer",
      meta: { className: "text-right" }
    }),
    columnHelper.accessor("attestationsCount", {
      header: "Attestations",
      cell: props => <Number value={props.getValue()} />,
      meta: { className: "text-right" }
    }),
    columnHelper.accessor("depositsCount", {
      header: "Deposits",
      cell: props => <Number value={props.getValue()} />,
      meta: { className: "text-right" }
    }),
    columnHelper.accessor(
      (row, rowIndex) => { return { p: row.proposerSlashingsCount, a: row.attesterSlashingsCount } },
      {
        header: "Slashings P / A",
        cell: props => <>
          <Number value={props.getValue().p} /> / <Number value={props.getValue().a} />
        </>,
        meta: { className: "text-right" }
      }
    ),
    columnHelper.accessor("voluntaryExitsCount", {
      header: "Exits",
      cell: props => <Number value={props.getValue()} />,
      meta: { className: "text-right" }
    })
  ];

  const table = useDataTable(app, "blocks", kind, getBlock, getBlockRangePaths, columns, blocksCount);

  return (
    <DataTable table={table} updatable={true} />
  )
}