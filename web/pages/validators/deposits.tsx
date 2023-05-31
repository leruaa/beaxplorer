import DataTable from "../../components/data-table";
import Number from "../../components/number";
import Ethers from "../../components/ethers";
import Trim from "../../components/trim";
import * as Breadcrumb from "../../components/breadcrumb";
import { createColumnHelper } from "@tanstack/react-table";
import { Accent, AccentContext } from "../../hooks/accent";
import { useDataTable } from "../../hooks/data";
import { UsersThree } from "@phosphor-icons/react";
import { App, ExecutionLayerDepositView, getExecutionLayerDeposit, getExecutionLayerDepositMetaPath, getExecutionLayerDepositRangePaths, getMeta } from "../../pkg/web";



export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const metaPath = getExecutionLayerDepositMetaPath(app);
  const meta = await fetch(metaPath)
    .then(r => r.blob())
    .then(b => b.arrayBuffer())
    .then(a => getMeta(a));
  return {
    props: {
      executionLayerDepositsCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const columnHelper = createColumnHelper<ExecutionLayerDepositView>()

  const columns = [
    columnHelper.accessor("index", {
      header: "Index",
    }),
    columnHelper.accessor("blockNumber", {
      header: "Block number",
    }),
    columnHelper.accessor("publicKey", {
      header: "Public key",
      cell: props => <><Trim className="font-mono" value={props.getValue()} regEx={/^(.{10}).*$/g} groups={"$1"} />&hellip;</>
    }),
    columnHelper.accessor("amount", {
      header: "Amount",
    })
  ];

  const table = useDataTable(app, "deposits", { kind: "integers" }, getExecutionLayerDeposit, getExecutionLayerDepositRangePaths, columns, props.executionLayerDepositsCount);

  return (
    <AccentContext.Provider value={Accent.Purple}>
      <Breadcrumb.Root>
        <Breadcrumb.Text>
          <UsersThree />&nbsp;Execution layer deposits
        </Breadcrumb.Text>
      </Breadcrumb.Root>
      <section>
        <DataTable table={table} updatable={true} />
      </section>
    </AccentContext.Provider>
  )
}
