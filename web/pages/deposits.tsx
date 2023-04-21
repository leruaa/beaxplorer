import DataTable from "../components/data-table";
import Number from "../components/number";
import Breadcrumb from "../components/breadcrumb";
import useDataTable from "../hooks/data-table";
import { App, getDepositMeta, getDeposit, DepositView } from "../pkg";
import { createColumnHelper } from "@tanstack/react-table";
import Link from 'next/link';
import Trim from "../components/trim";


export async function getStaticProps() {
  const app = new App("http://localhost:3000");
  const meta = await getDepositMeta(app);
  return {
    props: {
      depositsCount: meta.count
    }
  }
}

export default (props) => {
  const app = new App(process.env.NEXT_PUBLIC_HOST);
  const columnHelper = createColumnHelper<DepositView>()

  const columns = [
    columnHelper.accessor("slot", {
      header: "Slot",
      cell: props => <Link href={`/block/${props.getValue()}`}>
        {props.getValue()}
      </Link>
    }),
    columnHelper.accessor("publicKey", {
      header: "Public key",
      cell: props => props.getValue()
    }),
    columnHelper.accessor("amount", {
      header: "Amount",
      cell: props => <Number value={props.getValue()} />
    }),
    columnHelper.accessor("signature", {
      header: "Signature",
      cell: props => <>
        <Trim value={props.getValue()} className="font-mono" regEx={/^(.{10}).*$/g} groups={"$1"} />&hellip;
      </>
    }),
  ];

  const table = useDataTable(app, "deposits", getDeposit, columns, props.depositsCount);

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
