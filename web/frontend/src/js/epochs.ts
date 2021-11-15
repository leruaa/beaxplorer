import { html } from "gridjs";
import { paginate } from "./pagination";

paginate({
  wrapperId: "epochs-list",
  apiUrl: "/api/epochs",
  columns: [
    {
      id: "epoch",
      name: "Epoch",
      formatter: (cell: any) => html(`<a href="/epoch/${cell}">${cell}</a>`)
    },
    {
      id: "ag0",
      name: "Time"
    },
    {
      id: "attestations_count",
      name: "Attestations"
    },
    {
      id: "deposits_count",
      name: "Deposits",
    },
    {
      id: "attester_slashings_count",
      name: "Slashings P / A",
    },
    {
      id: "finalized",
      name: "Finalized"
    },
    {
      id: "eligible_ether",
      name: "Eligible",
      formatter: (cell: any) => `${cell} ETH`
    },
    {
      id: "voted_ether",
      name: "Voted",
      formatter: (cell: any) => `${cell} ETH`
    },
    {
      id: "global_participation_rate",
      name: "Rate",
      formatter: (cell: any) => `${cell}%`
    }
  ],
  dataMapping: (data: any) => data.results.map(
    (e: any) => [
      e.epoch,
      e.ago,
      e.attestations_count,
      e.deposits_count,
      `${e.proposer_slashings_count} / ${e.attester_slashings_count}`,
      `${e.finalized ? "Yes" : "No"}`,
      e.eligible_ether,
      e.voted_ether,
      e.global_participation_percentage
    ]
  )
});
