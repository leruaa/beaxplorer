import DataTable from "../components/data-table";
import Number from "../components/number";
import Ethers from "../components/ethers";
import Trim from "../components/trim";
import * as Breadcrumb from "../components/breadcrumb";
import { App, getValidatorMetaPath, getMeta, ValidatorView, getValidator, getValidatorRangePaths } from "../pkg/web";
import { createColumnHelper } from "@tanstack/react-table";
import { Accent, AccentContext } from "../hooks/accent";
import { useDataTable } from "../hooks/data";
import { UsersThree } from "@phosphor-icons/react";



export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const metaPath = getValidatorMetaPath(app);
  const meta = await fetch(metaPath)
    .then(r => r.blob())
    .then(b => b.arrayBuffer())
    .then(a => getMeta(a));
  return {
    props: {
      validatorsCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const columnHelper = createColumnHelper<ValidatorView>()

  const columns = [
    columnHelper.accessor("pubkey", {
      header: "Public key",
      cell: props => <><Trim value={props.getValue()} regEx={/^(.{10}).*$/g} groups={"$1"} />&hellip;</>
    }),
    columnHelper.accessor("validator_index", {
      header: "Index",
      cell: props => <a href={`/validator/${props.getValue()}`}><Number value={props.getValue()} /></a>
    }),
    columnHelper.accessor("balance", {
      header: "Balance",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("activationEpoch", {
      header: "Activation",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("exitEpoch", {
      header: "Exit",
      cell: props => <Ethers value={props.getValue()} />
    }),
    columnHelper.accessor("withdrawableEpoch", {
      header: "Exit",
      cell: props => <Ethers value={props.getValue()} />
    })
  ];

  const table = useDataTable(app, "validators", { kind: "integers" }, getValidator, getValidatorRangePaths, columns, props.validatorsCount);

  return (
    <AccentContext.Provider value={Accent.Purple}>
      <Breadcrumb.Root>
        <Breadcrumb.Text>
          <UsersThree />&nbsp;Validators
        </Breadcrumb.Text>
      </Breadcrumb.Root>
      <section>
        <DataTable table={table} updatable={true} />
      </section>
    </AccentContext.Provider>
  )
}
