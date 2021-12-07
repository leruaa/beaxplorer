import { useMemo } from "react";
import DataTable from "../components/data-table";
import Breadcrumb from "../components/breadcrumb";

export default () => {

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

  const data = [
    {
      epoch: 1,
      attestations_count: 965
    },
    {
      epoch: 2,
      attestations_count: 486
    },
  ]

  return (
    <>
      <Breadcrumb breadcrumb={{ parts: [{ text: "Epochs", icon: "clock" }] }} />
      <section className="container mx-auto">
        <div className="tabular-data">
          <p>Showing epochs</p>
          <DataTable columns={useMemo(() => columns, [])} data={useMemo(() => data, [])}/>
        </div>
      </section>
    </>
  )
}
  