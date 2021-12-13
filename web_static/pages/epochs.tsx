import { useMemo } from "react";
import DataTable from "../components/data-table";
import Breadcrumb from "../components/breadcrumb";

export async function getServerSideProps(context) {
  const wasmModule = await import('../pkg');

  return {
    props: {
      epochs: await wasmModule.get_epochs("http://localhost:3000", context.query.page || "1")
    } 
  }
}

export default (props) => {

  const columns = [
    {
      accessor: "epoch",
      Header: "Epoch",
      Cell: ({ value }) => <a href={`/epoch/${value}`}>{value}</a>
    },
    {
      accessor: "ago",
      Header: "Time"
    },
    {
      accessor: "attestations_count",
      Header: "Attestations"
    },
    {
      accessor: "deposits_count",
      Header: "Deposits",
    },
    {
      accessor: "attester_slashings_count",
      Header: "Slashings P / A",
    },
    {
      accessor: "finalized",
      Header: "Finalized"
    },
    {
      accessor: "eligible_ether",
      Header: "Eligible"
    },
    {
      accessor: "voted_ether",
      Header: "Voted"
    },
    {
      accessor: "global_participation_rate",
      Header: "Rate"
    }
  ];

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing epochs</p>
          <DataTable columns={useMemo(() => columns, [])} data={useMemo(() => props.epochs, [])}/>
        </div>
      </section>
    </>
  )
}
  