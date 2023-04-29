import { createColumnHelper } from "@tanstack/react-table";
import { useDataTable } from "../../hooks/data";
import { App, BlockView, RangeKind, getBlock, getBlockPaths } from "../../pkg/web";
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
                cell: props => <a href={`/block/${props.getValue()}`}><Number value={props.getValue()} /></a>
            })
        );
    }

    columns = [
        ...columns,
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
            },
            enableSorting: false,
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

    const table = useDataTable(app, "blocks", kind, getBlock, getBlockPaths, columns, blocksCount);

    return (
        <DataTable table={table} />
    )
}